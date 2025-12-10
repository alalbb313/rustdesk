import 'dart:async';
import 'dart:math';
import 'dart:ui' as ui;

import 'package:bot_toast/bot_toast.dart';
import 'package:desktop_multi_window/desktop_multi_window.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart' hide TabBarTheme;
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:flutter_hbb/desktop/pages/remote_page.dart';
import 'package:flutter_hbb/desktop/pages/view_camera_page.dart';
import 'package:flutter_hbb/main.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:flutter_hbb/models/state_model.dart';
import 'package:get/get.dart';
import 'package:get/get_rx/src/rx_workers/utils/debouncer.dart';
import 'package:scroll_pos/scroll_pos.dart';
import 'package:window_manager/window_manager.dart';
import 'package:visibility_detector/visibility_detector.dart';

import '../../utils/multi_window_manager.dart';

const double _kTabBarHeight = kDesktopRemoteTabBarHeight;
const double _kIconSize = 18;
const double _kDividerIndent = 10;
const double _kActionIconSize = 12;

class TabInfo {
  final String key; // Notice: cm use client_id.toString() as key
  final String label;
  final IconData? selectedIcon;
  final IconData? unselectedIcon;
  final bool closable;
  final VoidCallback? onTabCloseButton;
  final VoidCallback? onTap;
  final Widget page;

  TabInfo(
      {required this.key,
      required this.label,
      this.selectedIcon,
      this.unselectedIcon,
      this.closable = true,
      this.onTabCloseButton,
      this.onTap,
      required this.page});
}

enum DesktopTabType {
  main,
  cm,
  remoteScreen,
  fileTransfer,
  viewCamera,
  portForward,
  terminal,
  install,
}

class DesktopTabState {
  final List<TabInfo> tabs = [];
  final ScrollPosController scrollController =
      ScrollPosController(itemCount: 0);
  final PageController pageController = PageController();
  int selected = 0;

  TabInfo get selectedTabInfo => tabs[selected];

  DesktopTabState() {
    scrollController.itemCount = tabs.length;
  }
}

CancelFunc showRightMenu(ToastBuilder builder,
    {BuildContext? context, Offset? target}) {
  return BotToast.showAttachedWidget(
    target: target,
    targetContext: context,
    verticalOffset: 0.0,
    horizontalOffset: 0.0,
    duration: Duration(seconds: 300),
    animationDuration: Duration(milliseconds: 0),
    animationReverseDuration: Duration(milliseconds: 0),
    preferDirection: PreferDirection.rightTop,
    ignoreContentClick: false,
    onlyOne: true,
    allowClick: true,
    enableSafeArea: true,
    backgroundColor: Color(0x00000000),
    attachedBuilder: builder,
  );
}

class DesktopTabController {
  final state = DesktopTabState().obs;
  final DesktopTabType tabType;

  /// index, key
  Function(int, String)? onRemoved;
  Function(String)? onSelected;

  DesktopTabController(
      {required this.tabType, this.onRemoved, this.onSelected});

  int get length => state.value.tabs.length;

  void add(TabInfo tab) {
    if (!isDesktop) return;
    final index = state.value.tabs.indexWhere((e) => e.key == tab.key);
    int toIndex;
    if (index >= 0) {
      toIndex = index;
    } else {
      state.update((val) {
        val!.tabs.add(tab);
      });
      state.value.scrollController.itemCount = state.value.tabs.length;
      toIndex = state.value.tabs.length - 1;
      assert(toIndex >= 0);
    }
    try {
      // tabPage has not been initialized, call `onSelected` at the end of initState
      jumpTo(toIndex, callOnSelected: false);
    } catch (e) {
      // call before binding controller will throw
      debugPrint("Failed to jumpTo: $e");
    }
  }

  void remove(int index) {
    if (!isDesktop) return;
    final len = state.value.tabs.length;
    if (index < 0 || index > len - 1) return;
    final key = state.value.tabs[index].key;
    final currentSelected = state.value.selected;
    int toIndex = 0;
    if (index == len - 1) {
      toIndex = max(0, currentSelected - 1);
    } else if (index < len - 1 && index < currentSelected) {
      toIndex = max(0, currentSelected - 1);
    }
    state.value.tabs.removeAt(index);
    state.value.scrollController.itemCount = state.value.tabs.length;
    jumpTo(toIndex);
    onRemoved?.call(index, key);
  }

  /// For addTab, tabPage has not been initialized, set [callOnSelected] to false,
  /// and call [onSelected] at the end of initState
  bool jumpTo(int index, {bool callOnSelected = true}) {
    if (!isDesktop || index < 0) {
      return false;
    }
    state.update((val) {
      if (val != null) {
        val.selected = index;
        Future.delayed(Duration(milliseconds: 100), (() {
          if (val.pageController.hasClients) {
            val.pageController.jumpToPage(index);
          }
          val.scrollController.itemCount = val.tabs.length;
          if (val.scrollController.hasClients &&
              val.scrollController.itemCount > index) {
            val.scrollController
                .scrollToItem(index, center: false, animate: true);
          }
        }));
      }
    });
    if ((isDesktop && (bind.isIncomingOnly() || bind.isOutgoingOnly())) ||
        callOnSelected) {
      if (state.value.tabs.length > index) {
        final key = state.value.tabs[index].key;
        onSelected?.call(key);
      }
    }
    return true;
  }

  bool jumpToByKey(String key, {bool callOnSelected = true}) =>
      jumpTo(state.value.tabs.indexWhere((tab) => tab.key == key),
          callOnSelected: callOnSelected);

  bool jumpToByKeyAndDisplay(String key, int display, {bool isCamera = false}) {
    for (int i = 0; i < state.value.tabs.length; i++) {
      final tab = state.value.tabs[i];
      if (tab.key == key) {
        final ffi = isCamera
            ? (tab.page as ViewCameraPage).ffi
            : (tab.page as RemotePage).ffi;
        if (ffi.ffiModel.pi.currentDisplay == display) {
          return jumpTo(i, callOnSelected: true);
        }
      }
    }
    return false;
  }

  void closeBy(String? key) {
    if (!isDesktop) return;
    assert(onRemoved != null);
    if (key == null) {
      if (state.value.selected < state.value.tabs.length) {
        remove(state.value.selected);
      }
    } else {
      final index = state.value.tabs.indexWhere((tab) => tab.key == key);
      remove(index);
    }
  }

  void clear() {
    state.value.tabs.clear();
    state.refresh();
  }

  Widget? widget(String key) {
    return state.value.tabs.firstWhereOrNull((tab) => tab.key == key)?.page;
  }
}

class TabThemeConf {
  double iconSize;

  TabThemeConf({required this.iconSize});
}

typedef TabBuilder = Widget Function(
    String key, Widget icon, Widget label, TabThemeConf themeConf);
typedef TabMenuBuilder = Widget Function(String key);
typedef LabelGetter = Rx<String> Function(String key);

/// [_lastClickTime], help to handle double click
int _lastClickTime = 0;

class DesktopTab extends StatefulWidget {
  final bool showLogo;
  final bool showTitle;
  final bool showMinimize;
  final bool showMaximize;
  final bool showClose;
  final Widget Function(Widget pageView)? pageViewBuilder;
  // Right click tab menu
  final TabMenuBuilder? tabMenuBuilder;
  final Widget? tail;
  final Future<bool> Function()? onWindowCloseButton;
  final TabBuilder? tabBuilder;
  final LabelGetter? labelGetter;
  final double? maxLabelWidth;
  final Color? selectedTabBackgroundColor;
  final Color? unSelectedTabBackgroundColor;
  final Color? selectedBorderColor;

  final DesktopTabController controller;

  final _scrollDebounce = Debouncer(delay: Duration(milliseconds: 50));

  final RxList<String> invisibleTabKeys = RxList.empty();

  DesktopTab({
    Key? key,
    required this.controller,
    this.showLogo = true,
    this.showTitle = false,
    this.showMinimize = true,
    this.showMaximize = true,
    this.showClose = true,
    this.pageViewBuilder,
    this.tabMenuBuilder,
    this.tail,
    this.onWindowCloseButton,
    this.tabBuilder,
    this.labelGetter,
    this.maxLabelWidth,
    this.selectedTabBackgroundColor,
    this.unSelectedTabBackgroundColor,
    this.selectedBorderColor,
  }) : super(key: key);

  static RxString tablabelGetter(String peerId) {
    final alias = bind.mainGetPeerOptionSync(id: peerId, key: 'alias');
    return RxString(getDesktopTabLabel(peerId, alias));
  }

  @override
  State<DesktopTab> createState() {
    return _DesktopTabState();
  }
}

// ignore: must_be_immutable
class _DesktopTabState extends State<DesktopTab>
    with MultiWindowListener, WindowListener {
  Timer? _macOSCheckRestoreTimer;
  int _macOSCheckRestoreCounter = 0;

  bool get showLogo => widget.showLogo;
  bool get showTitle => widget.showTitle;
  bool get showMinimize => widget.showMinimize;
  bool get showMaximize => widget.showMaximize;
  bool get showClose => widget.showClose;
  Widget Function(Widget pageView)? get pageViewBuilder =>
      widget.pageViewBuilder;
  TabMenuBuilder? get tabMenuBuilder => widget.tabMenuBuilder;
  Widget? get tail => widget.tail;
  Future<bool> Function()? get onWindowCloseButton =>
      widget.onWindowCloseButton;
  TabBuilder? get tabBuilder => widget.tabBuilder;
  LabelGetter? get labelGetter => widget.labelGetter;
  double? get maxLabelWidth => widget.maxLabelWidth;
  Color? get selectedTabBackgroundColor => widget.selectedTabBackgroundColor;
  Color? get unSelectedTabBackgroundColor =>
      widget.unSelectedTabBackgroundColor;
  Color? get selectedBorderColor => widget.selectedBorderColor;
  DesktopTabController get controller => widget.controller;
  RxList<String> get invisibleTabKeys => widget.invisibleTabKeys;
  Debouncer get _scrollDebounce => widget._scrollDebounce;

  Rx<DesktopTabState> get state => controller.state;

  DesktopTabType get tabType => controller.tabType;
  bool get isMainWindow =>
      tabType == DesktopTabType.main ||
      tabType == DesktopTabType.cm ||
      tabType == DesktopTabType.install;

  _DesktopTabState() : super();

  static RxString tablabelGetter(String peerId) {
    final alias = bind.mainGetPeerOptionSync(id: peerId, key: 'alias');
    return RxString(getDesktopTabLabel(peerId, alias));
  }

  @override
  void initState() {
    super.initState();
    DesktopMultiWindow.addListener(this);
    windowManager.addListener(this);

    Future.delayed(Duration(milliseconds: 500), () {
      if (isMainWindow) {
        windowManager.isMaximized().then((maximized) {
          if (stateGlobal.isMaximized.value != maximized) {
            WidgetsBinding.instance.addPostFrameCallback(
                (_) => setState(() => stateGlobal.setMaximized(maximized)));
          }
        });
      } else {
        final wc = WindowController.fromWindowId(kWindowId!);
        wc.isMaximized().then((maximized) {
          debugPrint("isMaximized $maximized");
          if (stateGlobal.isMaximized.value != maximized) {
            WidgetsBinding.instance.addPostFrameCallback(
                (_) => setState(() => stateGlobal.setMaximized(maximized)));
          }
        });
      }
    });
  }

  @override
  void dispose() {
    DesktopMultiWindow.removeListener(this);
    windowManager.removeListener(this);
    _macOSCheckRestoreTimer?.cancel();
    super.dispose();
  }

  void _setMaximized(bool maximize) {
    stateGlobal.setMaximized(maximize);
    _saveFrame();
    setState(() {});
  }

  @override
  void onWindowFocus() {
    stateGlobal.isFocused.value = true;
  }

  @override
  void onWindowBlur() {
    stateGlobal.isFocused.value = false;
  }

  @override
  void onWindowMinimize() {
    stateGlobal.setMinimized(true);
    stateGlobal.setMaximized(false);
    super.onWindowMinimize();
  }

  @override
  void onWindowMaximize() {
    stateGlobal.setMinimized(false);
    _setMaximized(true);
    super.onWindowMaximize();
  }

  @override
  void onWindowUnmaximize() {
    stateGlobal.setMinimized(false);
    _setMaximized(false);
    super.onWindowUnmaximize();
  }

  _saveFrame({bool? flush}) async {
    try {
      if (tabType == DesktopTabType.main) {
        await saveWindowPosition(WindowType.Main, flush: flush);
      } else if (kWindowType != null && kWindowId != null) {
        await saveWindowPosition(kWindowType!,
            windowId: kWindowId, flush: flush);
      }
    } catch (e) {
      debugPrint('Error saving window position: $e');
    }
  }

  @override
  void onWindowMoved() {
    _saveFrame();
    super.onWindowMoved();
  }

  @override
  void onWindowResized() {
    _saveFrame();
    super.onWindowResized();
  }

  @override
  void onWindowClose() async {
    mainWindowClose() async => await windowManager.hide();
    notMainWindowClose(WindowController windowController) async {
      if (controller.length != 0) {
        debugPrint("close not empty multiwindow from taskbar");
        if (isWindows) {
          await windowController.show();
          await windowController.focus();
          final res = await onWindowCloseButton?.call() ?? true;
          if (!res) return;
        }
        controller.clear();
      }
      await windowController.hide();
      await rustDeskWinManager
          .call(WindowType.Main, kWindowEventHide, {"id": kWindowId!});
    }

    macOSWindowClose(
      Future<bool> Function() checkFullscreen,
      Future<void> Function() closeFunc,
    ) async {
      _macOSCheckRestoreCounter = 0;
      _macOSCheckRestoreTimer =
          Timer.periodic(Duration(milliseconds: 30), (timer) async {
        _macOSCheckRestoreCounter++;
        if (!await checkFullscreen() || _macOSCheckRestoreCounter >= 30) {
          _macOSCheckRestoreTimer?.cancel();
          _macOSCheckRestoreTimer = null;
          Timer(Duration(milliseconds: 700), () async => await closeFunc());
        }
      });
    }

    await _saveFrame(flush: true);

    // hide window on close
    if (isMainWindow) {
      if (rustDeskWinManager.getActiveWindows().contains(kMainWindowId)) {
        await rustDeskWinManager.unregisterActiveWindow(kMainWindowId);
      }
      // macOS specific workaround, the window is not hiding when in fullscreen.
      if (isMacOS && await windowManager.isFullScreen()) {
        await windowManager.setFullScreen(false);
        await macOSWindowClose(
          () async => await windowManager.isFullScreen(),
          mainWindowClose,
        );
      } else {
        await mainWindowClose();
      }
    } else {
      // it's safe to hide the subwindow
      final controller = WindowController.fromWindowId(kWindowId!);
      if (isMacOS) {
        // onWindowClose() maybe called multiple times because of loopCloseWindow() in remote_tab_page.dart.
        // use ??=  to make sure the value is set on first call.

        if (await onWindowCloseButton?.call() ?? true) {
          if (await controller.isFullScreen()) {
            await controller.setFullscreen(false);
            stateGlobal.setFullscreen(false, procWnd: false);
            await macOSWindowClose(
              () async => await controller.isFullScreen(),
              () async => await notMainWindowClose(controller),
            );
          } else {
            await notMainWindowClose(controller);
          }
        }
      } else {
        await notMainWindowClose(controller);
      }
    }
    super.onWindowClose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      Obx(() {
        if (stateGlobal.showTabBar.isTrue &&
            !(kUseCompatibleUiMode && isHideSingleItem())) {
          final showBottomDivider = _showTabBarBottomDivider(tabType);
          return SizedBox(
            height: _kTabBarHeight,
            child: Column(
              children: [
                SizedBox(
                  height:
                      showBottomDivider ? _kTabBarHeight - 1 : _kTabBarHeight,
                  child: _buildBar(),
                ),
                if (showBottomDivider)
                  const Divider(
                    height: 1,
                  ),
              ],
            ),
          );
        } else {
          return Offstage();
        }
      }),
      Expanded(
          child: pageViewBuilder != null
              ? pageViewBuilder!(_buildPageView())
              : _buildPageView())
    ]);
  }

  List<Widget> _tabWidgets = [];
  Widget _buildPageView() {
    final child = Container(
        child: Obx(() => PageView(
            controller: state.value.pageController,
            physics: NeverScrollableScrollPhysics(),
            children: () {
              if (DesktopTabType.cm == tabType) {
                // Fix when adding a new tab still showing closed tabs with the same peer id, which would happen after the DesktopTab was stateful.
                return state.value.tabs.map((tab) {
                  return tab.page;
                }).toList();
              }

              /// to-do refactor, separate connection state and UI state for remote session.
              /// [workaround] PageView children need an immutable list, after it has been passed into PageView
              final tabLen = state.value.tabs.length;
              if (tabLen == _tabWidgets.length) {
                return _tabWidgets;
              } else if (_tabWidgets.isNotEmpty &&
                  tabLen == _tabWidgets.length + 1) {
                /// On add. Use the previous list(pointer) to prevent item's state init twice.
                /// *[_tabWidgets.isNotEmpty] means TabsWindow(remote_tab_page or file_manager_tab_page) opened before, but was hidden. In this case, we have to reload, otherwise the child can't be built.
                _tabWidgets.add(state.value.tabs.last.page);
                return _tabWidgets;
              } else {
                /// On remove or change. Use new list(pointer) to reload list children so that items loading order is normal.
                /// the Widgets in list must enable [AutomaticKeepAliveClientMixin]
                final newList = state.value.tabs.map((v) => v.page).toList();
                _tabWidgets = newList;
                return newList;
              }
            }())));
    if (tabType == DesktopTabType.remoteScreen) {
      return Container(color: kColorCanvas, child: child);
    } else {
      return child;
    }
  }

  /// Check whether to show ListView
  ///
  /// Conditions:
  /// - hide single item when only has one item (home) on [DesktopTabPage].
  bool isHideSingleItem() {
    return state.value.tabs.length == 1 &&
        (controller.tabType == DesktopTabType.main ||
            controller.tabType == DesktopTabType.install);
  }

  Widget _buildBar() {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Expanded(
            child: GestureDetector(
                // custom double tap handler
                onTap: !(bind.isIncomingOnly() && isInHomePage()) &&
                        showMaximize
                    ? () {
                        final current = DateTime.now().millisecondsSinceEpoch;
                        final elapsed = current - _lastClickTime;
                        _lastClickTime = current;
                        if (elapsed < bind.getDoubleClickTime()) {
                          // onDoubleTap
                          toggleMaximize(isMainWindow)
                              .then((value) => stateGlobal.setMaximized(value));
                        }
                      }
                    : null,
                onPanStart: (_) => startDragging(isMainWindow),
                onPanCancel: () {
                  // We want to disable dragging of the tab area in the tab bar.
                  // Disable dragging is needed because macOS handles dragging by default.
                  if (isMacOS) {
                    setMovable(isMainWindow, false);
                  }
                },
                onPanEnd: (_) {
                  if (isMacOS) {
                    setMovable(isMainWindow, false);
                  }
                },
                child: Row(
                  children: [
                    Offstage(
                        offstage: !isMacOS,
                        child: const SizedBox(
                          width: 78,
                        )),
                    Offstage(
                      offstage: kUseCompatibleUiMode || isMacOS,
                      child: Row(children: [
                        Offstage(
                          offstage: !showLogo,
                          child: loadIcon(16),
                        ),
                        Offstage(
                            offstage: !showTitle,
                            child: const Text(
                              "RustDesk",
                              style: TextStyle(fontSize: 13),
                            ).marginOnly(left: 2))
                      ]).marginOnly(
                        left: 5,
                        right: 10,
                      ),
                    ),
                    Expanded(
                        child: Listener(
                            // handle mouse wheel
                            onPointerSignal: (e) {
                              if (e is PointerScrollEvent) {
                                final sc =
                                    controller.state.value.scrollController;
                                if (!sc.canScroll) return;
                                _scrollDebounce.call(() {
                                  double adjust = 2.5;
                                  sc.animateTo(
                                      sc.offset + e.scrollDelta.dy * adjust,
                                      duration: Duration(milliseconds: 200),
                                      curve: Curves.ease);
                                });
                              }
                            },
                            child: _ListView(
                              controller: controller,
                              invisibleTabKeys: invisibleTabKeys,
                              tabBuilder: tabBuilder,
                              tabMenuBuilder: tabMenuBuilder,
                              labelGetter: labelGetter,
                              maxLabelWidth: maxLabelWidth,
                              selectedTabBackgroundColor:
                                  selectedTabBackgroundColor,
                              unSelectedTabBackgroundColor:
                                  unSelectedTabBackgroundColor,
                              selectedBorderColor: selectedBorderColor,
                            ))),
                  ],
                ))),
        // hide simulated action buttons when we in compatible ui mode, because of reusing system title bar.
        WindowActionPanel(
          isMainWindow: isMainWindow,
          state: state,
          tabController: controller,
          invisibleTabKeys: invisibleTabKeys,
          tail: tail,
          showMinimize: showMinimize,
          showMaximize: showMaximize,
          showClose: showClose,
          onClose: onWindowCloseButton,
          labelGetter: labelGetter,
        ).paddingOnly(left: 10)
      ],
    );
  }
}

class WindowActionPanel extends StatefulWidget {
  final bool isMainWindow;
  final Rx<DesktopTabState> state;
  final DesktopTabController tabController;

  final bool showMinimize;
  final bool showMaximize;
  final bool showClose;
  final Widget? tail;
  final Future<bool> Function()? onClose;

  final RxList<String> invisibleTabKeys;
  final LabelGetter? labelGetter;

  const WindowActionPanel(
      {Key? key,
      required this.isMainWindow,
      required this.state,
      required this.tabController,
      required this.invisibleTabKeys,
      this.tail,
      this.showMinimize = true,
      this.showMaximize = true,
      this.showClose = true,
      this.onClose,
      this.labelGetter})
      : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return WindowActionPanelState();
  }
}

class WindowActionPanelState extends State<WindowActionPanel> {
  bool showTabDowndown() {
    return widget.tabController.state.value.tabs.length > 1 &&
        (widget.tabController.tabType == DesktopTabType.remoteScreen ||
            widget.tabController.tabType == DesktopTabType.fileTransfer ||
            widget.tabController.tabType == DesktopTabType.viewCamera ||
            widget.tabController.tabType == DesktopTabType.portForward ||
            widget.tabController.tabType == DesktopTabType.cm);
  }

  List<String> existingInvisibleTab() {
    return widget.invisibleTabKeys
        .where((key) =>
            widget.tabController.state.value.tabs.any((tab) => tab.key == key))
        .toList();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        Obx(() {
          if (showTabDowndown() && existingInvisibleTab().isNotEmpty) {
            return _TabDropDownButton(
                controller: widget.tabController,
                labelGetter: widget.labelGetter,
                tabkeys: existingInvisibleTab());
          } else {
            return Offstage();
          }
        }),
        if (widget.tail != null) widget.tail!,
        if (!kUseCompatibleUiMode)
          Row(
            children: [
              if (widget.showMinimize && !isMacOS)
                ActionIcon(
                  message: 'Minimize',
                  icon: IconFont.min,
                  onTap: () {
                    if (widget.isMainWindow) {
                      windowManager.minimize();
                    } else {
                      WindowController.fromWindowId(kWindowId!).minimize();
                    }
                  },
                  isClose: false,
                ),
              if (widget.showMaximize && !isMacOS)
                Obx(() => ActionIcon(
                      message: stateGlobal.isMaximized.isTrue
                          ? 'Restore'
                          : 'Maximize',
                      icon: stateGlobal.isMaximized.isTrue
                          ? IconFont.restore
                          : IconFont.max,
                      onTap: bind.isIncomingOnly() && isInHomePage()
                          ? null
                          : _toggleMaximize,
                      isClose: false,
                    )),
              if (widget.showClose && !isMacOS)
                ActionIcon(
                  message: 'Close',
                  icon: IconFont.close,
                  onTap: () async {
                    final res = await widget.onClose?.call() ?? true;
                    if (res) {
                      // hide for all window
                      // note: the main window can be restored by tray icon
                      Future.delayed(Duration.zero, () async {
                        if (widget.isMainWindow) {
                          await windowManager.close();
                        } else {
                          await WindowController.fromWindowId(kWindowId!)
                              .close();
                        }
                      });
                    }
                  },
                  isClose: true,
                )
            ],
          ),
      ],
    );
  }

  void _toggleMaximize() {
    toggleMaximize(widget.isMainWindow).then((maximize) {
      // update state for sub window, wc.unmaximize/maximize() will not invoke onWindowMaximize/Unmaximize
      stateGlobal.setMaximized(maximize);
    });
  }
}

void startDragging(bool isMainWindow) {
  if (isMainWindow) {
    windowManager.startDragging();
  } else {
    WindowController.fromWindowId(kWindowId!).startDragging();
  }
}

void setMovable(bool isMainWindow, bool movable) {
  if (isMainWindow) {
    windowManager.setMovable(movable);
  } else {
    WindowController.fromWindowId(kWindowId!).setMovable(movable);
  }
}

/// return true -> window will be maximize
/// return false -> window will be unmaximize
Future<bool> toggleMaximize(bool isMainWindow) async {
  if (isMainWindow) {
    if (await windowManager.isMaximized()) {
      windowManager.unmaximize();
      return false;
    } else {
      windowManager.maximize();
      return true;
    }
  } else {
    final wc = WindowController.fromWindowId(kWindowId!);
    if (await wc.isMaximized()) {
      wc.unmaximize();
      return false;
    } else {
      wc.maximize();
      return true;
    }
  }
}

Future<bool> closeConfirmDialog() async {
  var confirm = true;
  final res = await gFFI.dialogManager.show<bool>((setState, close, context) {
    submit() {
      String value = bool2option(kOptionEnableConfirmClosingTabs, confirm);
      bind.mainSetLocalOption(
          key: kOptionEnableConfirmClosingTabs, value: value);
      close(true);
    }

    return CustomAlertDialog(
      title: Row(children: [
        const Icon(Icons.warning_amber_sharp,
            color: Colors.redAccent, size: 20),
        const SizedBox(width: 10),
        Text(translate('Warning')),
      ]),
      content: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(translate('Close all?')),
          const SizedBox(height: 10),
          Row(
            children: [
              Checkbox(
                value: confirm,
                onChanged: (v) {
                  setState(() {
                    confirm = v!;
                  });
                },
              ),
              Text(translate('Don\'t show again')),
            ],
          ),
        ],
      ),
      actions: [
        dialogButton('Cancel', onPressed: () => close(false), isOutline: true),
        dialogButton('OK', onPressed: submit),
      ],
      onSubmit: submit,
      onCancel: () => close(false),
    );
  });
  return res ?? false;
}

class ActionIcon extends StatefulWidget {
  final String message;
  final IconData icon;
  final VoidCallback? onTap;
  final bool isClose;

  const ActionIcon(
      {Key? key,
      required this.message,
      required this.icon,
      required this.onTap,
      required this.isClose})
      : super(key: key);

  @override
  State<ActionIcon> createState() => _ActionIconState();
}

class _ActionIconState extends State<ActionIcon> {
  bool _isHover = false;

  @override
  Widget build(BuildContext context) {
    return Tooltip(
        message: translate(widget.message),
        child: InkWell(
          onTap: widget.onTap,
          onHover: (value) {
            setState(() {
              _isHover = value;
            });
          },
          child: Container(
            width: 40,
            height: _kTabBarHeight,
            decoration: BoxDecoration(
              color: _isHover
                  ? (widget.isClose ? Colors.red : Colors.grey.withOpacity(0.2))
                  : null,
            ),
            child: Icon(
              widget.icon,
              size: _kActionIconSize,
              color: _isHover && widget.isClose ? Colors.white : null,
            ),
          ),
        ));
  }
}

class _ListView extends StatefulWidget {
  final DesktopTabController controller;
  final RxList<String> invisibleTabKeys;
  final TabBuilder? tabBuilder;
  final TabMenuBuilder? tabMenuBuilder;
  final LabelGetter? labelGetter;
  final double? maxLabelWidth;
  final Color? selectedTabBackgroundColor;
  final Color? unSelectedTabBackgroundColor;
  final Color? selectedBorderColor;

  const _ListView({
    Key? key,
    required this.controller,
    required this.invisibleTabKeys,
    this.tabBuilder,
    this.tabMenuBuilder,
    this.labelGetter,
    this.maxLabelWidth,
    this.selectedTabBackgroundColor,
    this.unSelectedTabBackgroundColor,
    this.selectedBorderColor,
  }) : super(key: key);

  @override
  State<_ListView> createState() => _ListViewState();
}

class _ListViewState extends State<_ListView> {
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    widget.controller.state.value.scrollController.groupController =
        _scrollController;
  }

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      final state = widget.controller.state.value;
      return VisibilityDetector(
          key: Key('tabbar_list_view'),
          onVisibilityChanged: (info) {
            if (info.visibleFraction == 1.0) {
              if (state.scrollController.hasClients &&
                  state.scrollController.itemCount > state.selected) {
                state.scrollController
                    .scrollToItem(state.selected, center: false, animate: true);
              }
            }
          },
          child: ListView.builder(
              controller: _scrollController,
              scrollDirection: Axis.horizontal,
              itemCount: state.tabs.length,
              itemBuilder: (context, index) {
                return _TabItem(
                  index: index,
                  controller: widget.controller,
                  invisibleTabKeys: widget.invisibleTabKeys,
                  tabBuilder: widget.tabBuilder,
                  tabMenuBuilder: widget.tabMenuBuilder,
                  labelGetter: widget.labelGetter,
                  maxLabelWidth: widget.maxLabelWidth,
                  selectedTabBackgroundColor: widget.selectedTabBackgroundColor,
                  unSelectedTabBackgroundColor:
                      widget.unSelectedTabBackgroundColor,
                  selectedBorderColor: widget.selectedBorderColor,
                );
              }));
    });
  }
}

class _TabItem extends StatefulWidget {
  final int index;
  final DesktopTabController controller;
  final RxList<String> invisibleTabKeys;
  final TabBuilder? tabBuilder;
  final TabMenuBuilder? tabMenuBuilder;
  final LabelGetter? labelGetter;
  final double? maxLabelWidth;
  final Color? selectedTabBackgroundColor;
  final Color? unSelectedTabBackgroundColor;
  final Color? selectedBorderColor;

  const _TabItem({
    Key? key,
    required this.index,
    required this.controller,
    required this.invisibleTabKeys,
    this.tabBuilder,
    this.tabMenuBuilder,
    this.labelGetter,
    this.maxLabelWidth,
    this.selectedTabBackgroundColor,
    this.unSelectedTabBackgroundColor,
    this.selectedBorderColor,
  }) : super(key: key);

  @override
  State<_TabItem> createState() => _TabItemState();
}

class _TabItemState extends State<_TabItem> {
  bool _isHover = false;

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      final state = widget.controller.state.value;
      // check if index is valid
      if (widget.index >= state.tabs.length) return Offstage();
      final tab = state.tabs[widget.index];
      final selected = state.selected == widget.index;
      final showClose = _isHover || selected;
      final label = widget.labelGetter?.call(tab.key) ?? tab.label.obs;
      return VisibilityDetector(
          key: Key(tab.key),
          onVisibilityChanged: (info) {
            if (info.visibleFraction < 1.0) {
              if (!widget.invisibleTabKeys.contains(tab.key)) {
                widget.invisibleTabKeys.add(tab.key);
              }
            } else {
              widget.invisibleTabKeys.remove(tab.key);
            }
          },
          child: GestureDetector(
              onTap: () {
                widget.controller.jumpTo(widget.index);
                tab.onTap?.call();
              },
              onSecondaryTapUp: (details) {
                if (widget.tabMenuBuilder != null) {
                  showRightMenu(
                      (cancelFunc) => widget.tabMenuBuilder!(tab.key),
                      target: details.globalPosition);
                }
              },
              child: MouseRegion(
                  onEnter: (event) {
                    setState(() {
                      _isHover = true;
                    });
                  },
                  onExit: (event) {
                    setState(() {
                      _isHover = false;
                    });
                  },
                  child: Container(
                    decoration: BoxDecoration(
                      color: selected
                          ? widget.selectedTabBackgroundColor ??
                              Theme.of(context).cardColor
                          : _isHover
                              ? (widget.unSelectedTabBackgroundColor ??
                                      Theme.of(context).canvasColor)
                                  .withOpacity(0.8)
                              : widget.unSelectedTabBackgroundColor ??
                                  Theme.of(context).canvasColor,
                      border: Border(
                          right: BorderSide(
                              color: Theme.of(context).dividerColor,
                              width: 1.0),
                          top: selected
                              ? BorderSide(
                                  color: widget.selectedBorderColor ??
                                      Theme.of(context).primaryColor,
                                  width: 2.0)
                              : BorderSide.none),
                    ),
                    padding: EdgeInsets.only(
                        left: _kDividerIndent,
                        right: tab.closable ? 0 : _kDividerIndent),
                    constraints: BoxConstraints(
                        minWidth: 100,
                        maxWidth: widget.maxLabelWidth ?? 240.0),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Expanded(
                            child: Row(
                          children: [
                            if (widget.tabBuilder != null)
                              widget.tabBuilder!(
                                  tab.key,
                                  _buildIcon(tab, selected),
                                  _buildLabel(label.value),
                                  TabThemeConf(iconSize: _kIconSize))
                            else ...[
                              _buildIcon(tab, selected),
                              SizedBox(width: 8),
                              Expanded(child: _buildLabel(label.value))
                            ]
                          ],
                        )),
                        if (tab.closable)
                          _TabCloseButton(
                            onTap: () {
                              if (tab.onTabCloseButton != null) {
                                tab.onTabCloseButton!();
                              } else {
                                widget.controller.remove(widget.index);
                              }
                            },
                            show: showClose,
                          )
                      ],
                    ),
                  ))));
    });
  }

  Widget _buildIcon(TabInfo tab, bool selected) {
    return Icon(
      selected
          ? tab.selectedIcon ?? tab.unselectedIcon
          : tab.unselectedIcon ?? tab.selectedIcon,
      size: _kIconSize,
      color: selected ? Theme.of(context).primaryColor : null,
    );
  }

  Widget _buildLabel(String label) {
    return Text(
      label,
      overflow: TextOverflow.ellipsis,
      style: TextStyle(fontSize: 13),
    );
  }
}

class _TabCloseButton extends StatefulWidget {
  final VoidCallback onTap;
  final bool show;

  const _TabCloseButton({Key? key, required this.onTap, required this.show})
      : super(key: key);

  @override
  State<_TabCloseButton> createState() => _TabCloseButtonState();
}

class _TabCloseButtonState extends State<_TabCloseButton> {
  bool _isHover = false;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: widget.onTap,
      onHover: (value) {
        setState(() {
          _isHover = value;
        });
      },
      child: Container(
        margin: EdgeInsets.symmetric(horizontal: 8),
        padding: EdgeInsets.all(2),
        decoration: BoxDecoration(
          color: _isHover ? Colors.grey.withOpacity(0.5) : null,
          borderRadius: BorderRadius.circular(4),
        ),
        child: Opacity(
          opacity: widget.show ? 1.0 : 0.0,
          child: Icon(
            Icons.close,
            size: 14,
          ),
        ),
      ),
    );
  }
}

class _TabDropDownButton extends StatelessWidget {
  final DesktopTabController controller;
  final List<String> tabkeys;
  final LabelGetter? labelGetter;

  const _TabDropDownButton(
      {Key? key,
      required this.controller,
      required this.tabkeys,
      this.labelGetter})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return PopupMenuButton<String>(
      tooltip: translate('Hidden Tabs'),
      icon: Icon(Icons.arrow_drop_down),
      padding: EdgeInsets.zero,
      itemBuilder: (context) {
        return tabkeys.map((key) {
          final tab =
              controller.state.value.tabs.firstWhere((element) => element.key == key);
          final label = labelGetter?.call(key) ?? tab.label.obs;
          return PopupMenuItem<String>(
            value: key,
            child: Obx(() => Text(
                  label.value,
                  style: TextStyle(fontSize: 13),
                )),
          );
        }).toList();
      },
      onSelected: (key) {
        controller.jumpToByKey(key);
      },
    );
  }
}

bool _showTabBarBottomDivider(DesktopTabType type) {
  return type == DesktopTabType.main ||
      type == DesktopTabType.install ||
      type == DesktopTabType.cm;
}

String getDesktopTabLabel(String peerId, String alias) {
  if (alias.isNotEmpty) {
    return alias;
  }
  return peerId;
}
