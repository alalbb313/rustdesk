#include "flutter_window.h"

#include <desktop_multi_window/desktop_multi_window_plugin.h>
#include <texture_rgba_renderer/texture_rgba_renderer_plugin_c_api.h>
#include <flutter_gpu_texture_renderer/flutter_gpu_texture_renderer_plugin_c_api.h>

#include "flutter/generated_plugin_registrant.h"

#include <flutter/event_channel.h>
#include <flutter/event_sink.h>
#include <flutter/event_stream_handler_functions.h>
#include <flutter/method_channel.h>
#include <flutter/standard_method_codec.h>

#include <windows.h>

#include <optional>
#include <memory>

#include "win32_desktop.h"

namespace {

void RegisterHostChannel(flutter::BinaryMessenger* messenger, HWND hwnd) {
  auto channel = std::make_unique<flutter::MethodChannel<>>(
      messenger, "org.rustdesk.rustdesk/host",
      &flutter::StandardMethodCodec::GetInstance());

  channel->SetMethodCallHandler(
      [hwnd](const flutter::MethodCall<>& call, std::unique_ptr<flutter::MethodResult<>> result) {
        if (call.method_name() == "bumpMouse") {
          auto arguments = call.arguments();
          int dx = 0, dy = 0;

          if (std::holds_alternative<flutter::EncodableMap>(*arguments)) {
            auto argsMap = std::get<flutter::EncodableMap>(*arguments);
            auto dxIt = argsMap.find(flutter::EncodableValue("dx"));
            auto dyIt = argsMap.find(flutter::EncodableValue("dy"));

            if ((dxIt != argsMap.end()) && std::holds_alternative<int>(dxIt->second)) {
              dx = std::get<int>(dxIt->second);
            }
            if ((dyIt != argsMap.end()) && std::holds_alternative<int>(dyIt->second)) {
              dy = std::get<int>(dyIt->second);
            }
          } else if (std::holds_alternative<flutter::EncodableList>(*arguments)) {
            auto argsList = std::get<flutter::EncodableList>(*arguments);

            if ((argsList.size() >= 1) && std::holds_alternative<int>(argsList[0])) {
              dx = std::get<int>(argsList[0]);
            }
            if ((argsList.size() >= 2) && std::holds_alternative<int>(argsList[1])) {
              dy = std::get<int>(argsList[1]);
            }
          }

          bool succeeded = Win32Desktop::BumpMouse(dx, dy);
          result->Success(succeeded);
        } else if (call.method_name() == "setWindowContentSize") {
            auto arguments = call.arguments();
            if (std::holds_alternative<flutter::EncodableMap>(*arguments)) {
              auto argsMap = std::get<flutter::EncodableMap>(*arguments);
              auto widthIt = argsMap.find(flutter::EncodableValue("width"));
              auto heightIt = argsMap.find(flutter::EncodableValue("height"));
              auto leftIt = argsMap.find(flutter::EncodableValue("left"));
              auto topIt = argsMap.find(flutter::EncodableValue("top"));
              
              if (widthIt != argsMap.end() && heightIt != argsMap.end() &&
                  std::holds_alternative<double>(widthIt->second) &&
                  std::holds_alternative<double>(heightIt->second)) {
                
                int width = static_cast<int>(std::get<double>(widthIt->second));
                int height = static_cast<int>(std::get<double>(heightIt->second));
                
                LONG style = GetWindowLong(hwnd, GWL_STYLE);
                LONG exStyle = GetWindowLong(hwnd, GWL_EXSTYLE);
                
                RECT rect = {0, 0, width, height};
                
                if (AdjustWindowRectEx(&rect, style, FALSE, exStyle)) {
                   int w = rect.right - rect.left;
                   int h = rect.bottom - rect.top;
                   UINT flags = SWP_NOZORDER | SWP_NOACTIVATE;
                   int x = 0;
                   int y = 0;

                   if (leftIt != argsMap.end() && topIt != argsMap.end() &&
                       std::holds_alternative<double>(leftIt->second) &&
                       std::holds_alternative<double>(topIt->second)) {
                       x = static_cast<int>(std::get<double>(leftIt->second));
                       y = static_cast<int>(std::get<double>(topIt->second));
                   } else {
                       flags |= SWP_NOMOVE;
                   }

                   SetWindowPos(hwnd, NULL, x, y, w, h, flags);
                   result->Success(nullptr);
                   return;
                }
              }
            }
            result->Error("INVALID_ARGUMENTS", "Width and height are required");
        } else {
          result->NotImplemented();
        }
      });
}

} // namespace

FlutterWindow::FlutterWindow(const flutter::DartProject& project)
    : project_(project) {}

FlutterWindow::~FlutterWindow() {}

bool FlutterWindow::OnCreate() {
  if (!Win32Window::OnCreate()) {
    return false;
  }

  RECT frame = GetClientArea();

  // The size here must match the window dimensions to avoid unnecessary surface
  // creation / destruction in the startup path.
  flutter_controller_ = std::make_unique<flutter::FlutterViewController>(
      frame.right - frame.left, frame.bottom - frame.top, project_);
  // Ensure that basic setup of the controller was successful.
  if (!flutter_controller_->engine() || !flutter_controller_->view()) {
    return false;
  }
  RegisterPlugins(flutter_controller_->engine());

  RegisterHostChannel(flutter_controller_->engine()->messenger(), GetHandle());

  DesktopMultiWindowSetWindowCreatedCallback([](void *controller) {
    auto *flutter_view_controller =
        reinterpret_cast<flutter::FlutterViewController *>(controller);
    auto *registry = flutter_view_controller->engine();
    TextureRgbaRendererPluginCApiRegisterWithRegistrar(
        registry->GetRegistrarForPlugin("TextureRgbaRendererPlugin"));
    FlutterGpuTextureRendererPluginCApiRegisterWithRegistrar(
        registry->GetRegistrarForPlugin("FlutterGpuTextureRendererPluginCApi"));
    
    RegisterHostChannel(registry->messenger(), flutter_view_controller->view()->GetNativeWindow());
  });
  SetChildContent(flutter_controller_->view()->GetNativeWindow());
  return true;
}

void FlutterWindow::OnDestroy() {
  if (flutter_controller_) {
    flutter_controller_ = nullptr;
  }

  Win32Window::OnDestroy();
}

LRESULT
FlutterWindow::MessageHandler(HWND hwnd, UINT const message,
                              WPARAM const wparam,
                              LPARAM const lparam) noexcept {
  // Give Flutter, including plugins, an opportunity to handle window messages.
  if (flutter_controller_) {
    std::optional<LRESULT> result =
        flutter_controller_->HandleTopLevelWindowProc(hwnd, message, wparam,
                                                      lparam);
    if (result) {
      return *result;
    }
  }

  switch (message) {
    case WM_FONTCHANGE:
      flutter_controller_->engine()->ReloadSystemFonts();
      break;
  }

  return Win32Window::MessageHandler(hwnd, message, wparam, lparam);
}
