# RustDesk Custom Configuration Guide

This document records all custom modifications made to the RustDesk source code to achieve specific behaviors, such as hardcoded default servers, UI customization, and build fixes. Use this guide to re-apply changes after pulling updates from the upstream repository.

## 1. Core Configuration & Default Settings

### File: `libs/hbb_common/src/config.rs`

**Goal**: Set default values for settings and implement server/key fallback logic while hiding them in the UI.

#### 1.1. Modify Default Settings Map
Locate `DEFAULT_SETTINGS`, `DEFAULT_LOCAL_SETTINGS`, and `DEFAULT_DISPLAY_SETTINGS` and update/add entries:

```rust
// In DEFAULT_SETTINGS
"access-mode" => "full",
"direct-server" => "Y",
"allow-insecure-tls-fallback" => "Y",

// In DEFAULT_LOCAL_SETTINGS
"enable-udp-punch" => "Y",
"enable-ipv6-punch" => "Y",

// In DEFAULT_DISPLAY_SETTINGS
"collapse_toolbar" => "Y",
```

#### 1.2. Implement Hardcoded Server & Key Fallbacks
Modify/Add the following functions to return your hardcoded values when configuration is empty:

```rust
pub fn get_rendezvous_server() -> String {
    // ... existing logic ...
    if rendezvous_server.is_empty() {
        rendezvous_server = "rustdesk.alalbb.top".to_owned(); // CUSTOM
    }
    rendezvous_server
}

pub fn get_relay_server() -> String {
    let v = Self::get_option(keys::OPTION_RELAY_SERVER);
    if v.is_empty() {
        "rustdesk.alalbb.top".to_owned() // CUSTOM
    } else {
        v
    }
}

pub fn get_api_server() -> String {
    let v = Self::get_option(keys::OPTION_API_SERVER);
    if v.is_empty() {
        "https://rustdesk.alalbb.top:8443".to_owned() // CUSTOM
    } else {
        v
    }
}

pub fn get_key() -> String {
    let v = Self::get_option(keys::OPTION_KEY);
    if v.is_empty() {
        "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=".to_owned() // CUSTOM
    } else {
        v
    }
}
```

#### 1.3. Hide Default Values in UI
Modify `get_options` to filter out values that match your hardcoded defaults, so they appear blank in the UI:

```rust
pub fn get_options() -> Vec<String> {
    // ...
    // Inside the loop or filter logic:
    if key == "custom-rendezvous-server" && value == "rustdesk.alalbb.top" { return None; }
    if key == "relay-server" && value == "rustdesk.alalbb.top" { return None; }
    if key == "api-server" && value == "https://rustdesk.alalbb.top:8443" { return None; }
    if key == "key" && value == "rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=" { return None; }
    // ...
}
```

---

## 2. Server Connection Logic Fixes

### File: `src/common.rs`

**Goal**: Ensure the client uses the default key when no key is configured.

#### Modify `get_key` function:
```rust
pub async fn get_key(sync: bool) -> String {
    // ...
    // Change fallback logic:
    if key.is_empty() {
        key = Config::get_key(); // CUSTOM: Use Config::get_key() instead of RS_PUB_KEY
    }
    key
}
```

### File: `src/ipc.rs`

**Goal**: Ensure the client uses the default rendezvous server when IPC returns empty config (due to UI hiding).

#### Modify `get_rendezvous_server` function:
```rust
pub async fn get_rendezvous_server(ms_timeout: u64) -> (String, Vec<String>) {
    if let Ok(Some(v)) = get_config_async("rendezvous_server", ms_timeout).await {
        let mut urls = v.split(",");
        let a = urls.next().unwrap_or_default().to_owned();
        // CUSTOM: Add check for empty string
        if !a.is_empty() {
            let b: Vec<String> = urls.map(|x| x.to_owned()).collect();
            return (a, b);
        }
    }
    (
        Config::get_rendezvous_server(),
        Config::get_rendezvous_servers(),
    )
}
```

---

## 3. Flutter UI & Build Fixes

### File: `flutter/lib/common.dart`

**Goal**: Fix Dart build errors and set default UI options.

1.  **Fix Import Order**: Move all `import` statements to the very top of the file.
2.  **Fix Duplicate Declarations**: Remove duplicate `defaultOptionAccessMode` and `defaultOptionCollapseToolbar` at the bottom of the file.
3.  **Fix Forward Reference**: Simplify default option getters:
    ```dart
    String get defaultOptionAccessMode => 'full'; // CUSTOM
    String get defaultOptionCollapseToolbar => 'Y'; // CUSTOM
    ```

### File: `flutter/lib/models/server_model.dart`

**Goal**: Minimize the connection window immediately when a password connection is authorized.

#### Modify `_addTab` function:
```dart
void _addTab(Client client) {
    // ...
    // Only do the hidden task when on Desktop.
    if (client.authorized && isDesktop) {
      // CUSTOM: Immediately minimize window
      Future.delayed(Duration.zero, () {
        if (!hideCm) windowManager.minimize();
      });
    }
    // ...
}
```

---

## 4. GitHub Actions Workflows

### File: `.github/workflows/cleanup-workflows.yml`
**Goal**: Add workflow to clean up old runs.
(Copy content from current repo)

### File: `.github/workflows/delete-all-workflows.yml`
**Goal**: Add workflow to delete ALL runs (with safety check).
(Copy content from current repo)

---

## 5. Verification Steps

After applying these changes:

1.  **Build**: Run the GitHub Actions build (e.g., by pushing a tag).
2.  **Check UI**:
    *   Settings -> Network: Server fields should be **BLANK**.
    *   Settings -> General: UDP/IPv6 should be **CHECKED**.
    *   Settings -> Display: Collapse Toolbar should be **CHECKED**.
3.  **Check Connection**:
    *   Connect to another peer. It should work without "Signature mismatch".
    *   Connect with password. The window should **minimize immediately**.
