lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estat"),
        ("Your Desktop", "EL teu escriptori"),
        ("desk_tip", "Pots accedir al teu escriptori amb aquest ID i contrasenya."),
        ("Password", "Contrasenya"),
        ("Ready", "Llest"),
        ("Established", "Establert"),
        ("connecting_status", "Connexió a la xarxa RustDesk en progrés..."),
        ("Enable service", "Habilitar Servei"),
        ("Start service", "Iniciar Servei"),
        ("Service is running", "El servei s'està executant"),
        ("Service is not running", "El servei no s'està executant"),
        ("not_ready_status", "No està llest. Comprova la teva connexió"),
        ("Control Remote Desktop", "Controlar escriptori remot"),
        ("Transfer file", "Transferir arxiu"),
        ("Connect", "Connectar"),
        ("Recent sessions", "Sessions recents"),
        ("Address book", "Directori"),
        ("Confirmation", "Confirmació"),
        ("TCP tunneling", "Túnel TCP"),
        ("Remove", "Eliminar"),
        ("Refresh random password", "Actualitzar contrasenya aleatòria"),
        ("Set your own password", "Estableix la teva pròpia contrasenya"),
        ("Enable keyboard/mouse", "Habilitar teclat/ratolí"),
        ("Enable clipboard", "Habilitar portapapers"),
        ("Enable file transfer", "Habilitar transferència d'arxius"),
        ("Enable TCP tunneling", "Habilitar túnel TCP"),
        ("IP Whitelisting", "Direccions IP admeses"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import server config", "Importar configuració de servidor"),
        ("Export Server Config", "Exportar configuració del servidor"),
        ("Import server configuration successfully", "Configuració de servidor importada amb èxit"),
        ("Export server configuration successfully", "Configuració de servidor exportada con èxit"),
        ("Invalid server configuration", "Configuració de servidor incorrecta"),
        ("Clipboard is empty", "El portapapers està buit"),
        ("Stop service", "Aturar servei"),
        ("Change ID", "Canviar ID"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Només pots utilitzar caràcters a-z, A-Z, 0-9 e _ (guionet baix). El primer caràcter ha de ser a-z o A-Z. La longitut ha d'estar entre 6 i 16 caràcters."),
        ("Website", "Lloc web"),
        ("About", "Sobre"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Silenciar"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Entrada d'àudio"),
        ("Enhancements", "Millores"),
        ("Hardware Codec", "Còdec de hardware"),
        ("Adaptive bitrate", "Tasa de bits adaptativa"),
        ("ID Server", "Servidor de IDs"),
        ("Relay Server", "Servidor Relay"),
        ("API Server", "Servidor API"),
        ("invalid_http", "ha de començar amb http:// o https://"),
        ("Invalid IP", "IP incorrecta"),
        ("Invalid format", "Format incorrecte"),
        ("server_not_support", "Encara no és compatible amb el servidor"),
        ("Not available", "No disponible"),
        ("Too frequent", "Massa comú"),
        ("Cancel", "Cancel·lar"),
        ("Skip", "Saltar"),
        ("Close", "Tancar"),
        ("Retry", "Reintentar"),
        ("OK", ""),
        ("Password Required", "Es necessita la contrasenya"),
        ("Please enter your password", "Si us plau, introdueixi la seva contrasenya"),
        ("Remember password", "Recordar contrasenya"),
        ("Wrong Password", "Contrasenya incorrecta"),
        ("Do you want to enter again?", "Vol tornar a entrar?"),
        ("Connection Error", "Error de connexió"),
        ("Error", ""),
        ("Reset by the peer", "Reestablert pel peer"),
        ("Connecting...", "Connectant..."),
        ("Connection in progress. Please wait.", "Connexió en procés. Esperi."),
        ("Please try 1 minute later", "Torni a provar-ho d'aquí un minut"),
        ("Login Error", "Error d'inicio de sessió"),
        ("Successful", "Exitós"),
        ("Connected, waiting for image...", "Connectant, esperant imatge..."),
        ("Name", "Nom"),
        ("Type", "Tipus"),
        ("Modified", "Modificat"),
        ("Size", "Grandària"),
        ("Show Hidden Files", "Mostrar arxius ocults"),
        ("Receive", "Rebre"),
        ("Send", "Enviar"),
        ("Refresh File", "Actualitzar arxiu"),
        ("Local", ""),
        ("Remote", "Remot"),
        ("Remote Computer", "Ordinador remot"),
        ("Local Computer", "Ordinador local"),
        ("Confirm Delete", "Confirma eliminació"),
        ("Delete", "Eliminar"),
        ("Properties", "Propietats"),
        ("Multi Select", "Selecció múltiple"),
        ("Select All", "Selecciona-ho Tot"),
        ("Unselect All", "Deselecciona-ho Tot"),
        ("Empty Directory", "Directori buit"),
        ("Not an empty directory", "No és un directori buit"),
        ("Are you sure you want to delete this file?", "Estàs segur que vols eliminar aquest arxiu?"),
        ("Are you sure you want to delete this empty directory?", "Estàs segur que vols eliminar aquest directori buit?"),
        ("Are you sure you want to delete the file of this directory?", "Estàs segur que vols eliminar aquest arxiu d'aquest directori?"),
        ("Do this for all conflicts", "Fes això per a tots els conflictes"),
        ("This is irreversible!", "Això és irreversible!"),
        ("Deleting", "Eliminant"),
        ("files", "arxius"),
        ("Waiting", "Esperant"),
        ("Finished", "Acabat"),
        ("Speed", "Velocitat"),
        ("Custom Image Quality", "Qualitat d'imatge personalitzada"),
        ("Privacy mode", "Mode privat"),
        ("Block user input", "Bloquejar entrada d'usuari"),
        ("Unblock user input", "Desbloquejar entrada d'usuari"),
        ("Adjust Window", "Ajustar finestra"),
        ("Original", "Original"),
        ("Shrink", "Reduir"),
        ("Stretch", "Estirar"),
        ("Scrollbar", "Barra de desplaçament"),
        ("ScrollAuto", "Desplaçament automàtico"),
        ("Good image quality", "Bona qualitat d'imatge"),
        ("Balanced", "Equilibrat"),
        ("Optimize reaction time", "Optimitzar el temps de reacció"),
        ("Custom", "Personalitzat"),
        ("Show remote cursor", "Mostrar cursor remot"),
        ("Show quality monitor", "Mostrar qualitat del monitor"),
        ("Disable clipboard", "Deshabilitar portapapers"),
        ("Lock after session end", "Bloquejar després del final de la sessió"),
        ("Insert", "Inserir"),
        ("Insert Lock", "Inserir bloqueig"),
        ("Refresh", "Actualitzar"),
        ("ID does not exist", "L'ID no existeix"),
        ("Failed to connect to rendezvous server", "No es pot connectar al servidor rendezvous"),
        ("Please try later", "Siusplau provi-ho més tard"),
        ("Remote desktop is offline", "L'escriptori remot està desconecctat"),
        ("Key mismatch", "La clau no coincideix"),
        ("Timeout", "Temps esgotat"),
        ("Failed to connect to relay server", "No es pot connectar al servidor de relay"),
        ("Failed to connect via rendezvous server", "No es pot connectar a través del servidor de rendezvous"),
        ("Failed to connect via relay server", "No es pot connectar a través del servidor de relay"),
        ("Failed to make direct connection to remote desktop", "No s'ha pogut establir una connexió directa amb l'escriptori remot"),
        ("Set Password", "Configurar la contrasenya"),
        ("OS Password", "contrasenya del sistema operatiu"),
        ("install_tip", ""),
        ("Click to upgrade", "Clicar per actualitzar"),
        ("Click to download", "Clicar per descarregar"),
        ("Click to update", "Clicar per refrescar"),
        ("Configure", "Configurar"),
        ("config_acc", ""),
        ("config_screen", ""),
        ("Installing ...", "Instal·lant ..."),
        ("Install", "Instal·lar"),
        ("Installation", "Instal·lació"),
        ("Installation Path", "Ruta d'instal·lació"),
        ("Create start menu shortcuts", "Crear accessos directes al menú d'inici"),
        ("Create desktop icon", "Crear icona d'escriptori"),
        ("agreement_tip", ""),
        ("Accept and Install", "Acceptar i instal·lar"),
        ("End-user license agreement", "Acord de llicència d'usuario final"),
        ("Generating ...", "Generant ..."),
        ("Your installation is lower version.", "La seva instal·lació és una versión inferior."),
        ("not_close_tcp_tip", ""),
        ("Listening ...", "Escoltant..."),
        ("Remote Host", "Hoste remot"),
        ("Remote Port", "Port remot"),
        ("Action", "Acció"),
        ("Add", "Afegirr"),
        ("Local Port", "Port local"),
        ("Local Address", "Adreça Local"),
        ("Change Local Port", "Canviar Port Local"),
        ("setup_server_tip", ""),
        ("Too short, at least 6 characters.", "Massa curt, almenys 6 caràcters."),
        ("The confirmation is not identical.", "La confirmación no coincideix."),
        ("Permissions", "Permisos"),
        ("Accept", "Acceptar"),
        ("Dismiss", "Cancel·lar"),
        ("Disconnect", "Desconnectar"),
        ("Enable file copy and paste", "Permetre copiar i enganxar arxius"),
        ("Connected", "Connectat"),
        ("Direct and encrypted connection", "Connexió directa i xifrada"),
        ("Relayed and encrypted connection", "connexió retransmesa i xifrada"),
        ("Direct and unencrypted connection", "connexió directa i sense xifrar"),
        ("Relayed and unencrypted connection", "connexió retransmesa i sense xifrar"),
        ("Enter Remote ID", "Introduixi l'ID remot"),
        ("Enter your password", "Introdueixi la seva contrasenya"),
        ("Logging in...", "Iniciant sessió..."),
        ("Enable RDP session sharing", "Habilitar l'ús compartit de sessions RDP"),
        ("Auto Login", "Inici de sessió automàtic"),
        ("Enable direct IP access", "Habilitar accés IP directe"),
        ("Rename", "Renombrar"),
        ("Space", "Espai"),
        ("Create desktop shortcut", "Crear accés directe a l'escriptori"),
        ("Change Path", "Cnviar ruta"),
        ("Create Folder", "Crear carpeta"),
        ("Please enter the folder name", "Indiqui el nom de la carpeta"),
        ("Fix it", "Soluciona-ho"),
        ("Warning", "Avís"),
        ("Login screen using Wayland is not supported", "La pantalla d'inici de sessió amb Wayland no és compatible"),
        ("Reboot required", "Cal reiniciar"),
        ("Unsupported display server", "Servidor de visualització no compatible"),
        ("x11 expected", "x11 necessari"),
        ("Port", ""),
        ("Settings", "Ajustaments"),
        ("Username", " Nom d'usuari"),
        ("Invalid port", "Port incorrecte"),
        ("Closed manually by the peer", "Tancat manualment pel peer"),
        ("Enable remote configuration modification", "Habilitar modificació remota de configuració"),
        ("Run without install", "Executar sense instal·lar"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Connecta sempre a través de relay"),
        ("whitelist_tip", ""),
        ("Login", "Inicia sessió"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Sortir"),
        ("Tags", ""),
        ("Search ID", "Cerca ID"),
        ("whitelist_sep", ""),
        ("Add ID", "Afegir ID"),
        ("Add Tag", "Afegir tag"),
        ("Unselect all tags", "Deseleccionar tots els tags"),
        ("Network error", "Error de xarxa"),
        ("Username missed", "Nom d'usuari oblidat"),
        ("Password missed", "Contrasenya oblidada"),
        ("Wrong credentials", "Credencials incorrectes"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Editar tag"),
        ("Forget Password", "Contrasenya oblidada"),
        ("Favorites", "Preferits"),
        ("Add to Favorites", "Afegir a preferits"),
        ("Remove from Favorites", "Treure de preferits"),
        ("Empty", "Buit"),
        ("Invalid folder name", "Nom de carpeta incorrecte"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Discovered", "Descobert"),
        ("install_daemon_tip", ""),
        ("Remote ID", "ID remot"),
        ("Paste", "Enganxar"),
        ("Paste here?", "Enganxar aquí?"),
        ("Are you sure to close the connection?", "Estàs segur que vols tancar la connexió?"),
        ("Download new version", "Descarregar nova versió"),
        ("Touch mode", "Mode tàctil"),
        ("Mouse mode", "Mode ratolí"),
        ("One-Finger Tap", "Toqui amb un dit"),
        ("Left Mouse", "Ratolí esquerra"),
        ("One-Long Tap", "Toc llarg"),
        ("Two-Finger Tap", "Toqui amb dos dits"),
        ("Right Mouse", "Botó dret"),
        ("One-Finger Move", "Moviment amb un dir"),
        ("Double Tap & Move", "Toqui dos cops i mogui"),
        ("Mouse Drag", "Arrastri amb el ratolí"),
        ("Three-Finger vertically", "Tres dits verticalment"),
        ("Mouse Wheel", "Roda del ratolí"),
        ("Two-Finger Move", "Moviment amb dos dits"),
        ("Canvas Move", "Moviment del llenç"),
        ("Pinch to Zoom", "Pessiga per fer zoom"),
        ("Canvas Zoom", "Ampliar llenç"),
        ("Reset canvas", "Reestablir llenç"),
        ("No permission of file transfer", "No tens permís de transferència de fitxers"),
        ("Note", "Nota"),
        ("Connection", "connexió"),
        ("Share Screen", "Compartir pantalla"),
        ("Chat", "Xat"),
        ("Total", "Total"),
        ("items", "ítems"),
        ("Selected", "Seleccionat"),
        ("Screen Capture", "Captura de pantalla"),
        ("Input Control", "Control d'entrada"),
        ("Audio Capture", "Captura d'àudio"),
        ("File Connection", "connexió d'arxius"),
        ("Screen Connection", "connexió de pantalla"),
        ("Do you accept?", "Acceptes?"),
        ("Open System Setting", "Configuració del sistema obert"),
        ("How to get Android input permission?", "Com obtenir el permís d'entrada d'Android?"),
        ("android_input_permission_tip1", "Per a que un dispositiu remot controli el seu dispositiu Android amb el ratolí o tocs, cal permetre que RustDesk utilitzi el servei d' \"Accesibilitat\"."),
        ("android_input_permission_tip2", "Vagi a la pàgina de [Serveis instal·lats], activi el servici [RustDesk Input]."),
        ("android_new_connection_tip", "S'ha rebut una nova sol·licitud de control per al dispositiu actual."),
        ("android_service_will_start_tip", "Habilitar la captura de pantalla iniciarà el servei automàticament, i permetrà que altres dispositius sol·licitin una connexió des d'aquest dispositiu."),
        ("android_stop_service_tip", "Tancar el servei tancarà totes les connexions establertes."),
        ("android_version_audio_tip", "La versión actual de Android no admet la captura d'àudio, actualizi a Android 10 o superior."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Compte"),
        ("Overwrite", "Sobreescriure"),
        ("This file exists, skip or overwrite this file?", "Aquest arxiu ja existeix, ometre o sobreescriure l'arxiu?"),
        ("Quit", "Sortir"),
        ("Help", "Ajuda"),
        ("Failed", "Ha fallat"),
        ("Succeeded", "Aconseguit"),
        ("Someone turns on privacy mode, exit", "Algú ha activat el mode de privacitat, surti"),
        ("Unsupported", "No suportat"),
        ("Peer denied", "Peer denegat"),
        ("Please install plugins", "Instal·li complements"),
        ("Peer exit", "El peer ha sortit"),
        ("Failed to turn off", "Error en apagar"),
        ("Turned off", "Apagat"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Mantenir RustDesk com a servei en segon pla"),
        ("Ignore Battery Optimizations", "Ignorar optimizacions de la bateria"),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Connexió no disponible"),
        ("Legacy mode", "Mode heretat"),
        ("Map mode", "Mode mapa"),
        ("Translate mode", "Mode traduit"),
        ("Use permanent password", "Utilitzar contrasenya permament"),
        ("Use both passwords", "Utilitzar ambdues contrasenyas"),
        ("Set permanent password", "Establir contrasenya permament"),
        ("Enable remote restart", "Activar reinici remot"),
        ("Restart remote device", "Reiniciar dispositiu"),
        ("Are you sure you want to restart", "Està segur que vol reiniciar?"),
        ("Restarting remote device", "Reiniciant dispositiu remot"),
        ("remote_restarting_tip", "Dispositiu remot reiniciant, tanqui aquest missatge i tornis a connectar amb la contrasenya."),
        ("Copied", "Copiat"),
        ("Exit Fullscreen", "Sortir de la pantalla completa"),
        ("Fullscreen", "Pantalla completa"),
        ("Mobile Actions", "Accions mòbils"),
        ("Select Monitor", "Seleccionar monitor"),
        ("Control Actions", "Accions de control"),
        ("Display Settings", "Configuració de pantalla"),
        ("Ratio", "Relació"),
        ("Image Quality", "Qualitat d'imatge"),
        ("Scroll Style", "Estil de desplaçament"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Connexió directa"),
        ("Relay Connection", "Connexió Relay"),
        ("Secure Connection", "Connexió segura"),
        ("Insecure Connection", "Connexió insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptativa"),
        ("General", ""),
        ("Security", "Seguretat"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Fosc"),
        ("Light Theme", ""),
        ("Dark", "Fosc"),
        ("Light", "Clar"),
        ("Follow System", "Tema del sistema"),
        ("Enable hardware codec", "Habilitar còdec per hardware"),
        ("Unlock Security Settings", "Desbloquejar ajustaments de seguretat"),
        ("Enable audio", "Habilitar àudio"),
        ("Unlock Network Settings", "Desbloquejar Ajustaments de Xarxa"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Accés IP Directe"),
        ("Proxy", ""),
        ("Apply", "Aplicar"),
        ("Disconnect all devices?", "Desconnectar tots els dispositius?"),
        ("Clear", "Netejar"),
        ("Audio Input Device", "Dispositiu d'entrada d'àudio"),
        ("Use IP Whitelisting", "Utilitza llista de IPs admeses"),
        ("Network", "Xarxa"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "Gravant"),
        ("Directory", "Directori"),
        ("Automatically record incoming sessions", "Gravació automàtica de sessions entrants"),
        ("Change", "Canviar"),
        ("Start session recording", "Començar gravació de sessió"),
        ("Stop session recording", "Aturar gravació de sessió"),
        ("Enable recording session", "Habilitar gravació de sessió"),
        ("Enable LAN discovery", "Habilitar descobriment de LAN"),
        ("Deny LAN discovery", "Denegar descobriment de LAN"),
        ("Write a message", "Escriure un missatge"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Desconnectat"),
        ("Other", "Altre"),
        ("Confirm before closing multiple tabs", "Confirmar abans de tancar múltiples pestanyes"),
        ("Keyboard Settings", "Ajustaments de teclat"),
        ("Full Access", "Acces complet"),
        ("Screen Share", "Compartir pantalla"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requereix Ubuntu 21.04 o una versió superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requereix una versió superior de la distribución de Linux. Provi l'escriptori X11 o canvïi el seu sistema operatiu."),
        ("JumpLink", "Veure"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Seleccioni la pantalla que es compartirà (Operar al costat del peer)."),
        ("Show RustDesk", "Mostrar RustDesk"),
        ("This PC", "Aquest PC"),
        ("or", "o"),
        ("Continue with", "Continuar amb"),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to address book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("switch_display_elevated_connections_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("Follow remote cursor", ""),
        ("Follow remote window focus", ""),
    ].iter().cloned().collect();
}
