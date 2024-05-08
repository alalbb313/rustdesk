lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estado"),
        ("Your Desktop", "Ambiente de Trabalho"),
        ("desk_tip", "O seu Ambiente de Trabalho pode ser acedido com este ID e palavra-passe."),
        ("Password", "Senha"),
        ("Ready", "Pronto"),
        ("Established", "Estabelecido"),
        ("connecting_status", "A ligar à rede do RustDesk..."),
        ("Enable service", "Activar Serviço"),
        ("Start service", "Iniciar Serviço"),
        ("Service is running", "Serviço está activo"),
        ("Service is not running", "Serviço não está activo"),
        ("not_ready_status", "Indisponível. Por favor verifique a sua ligação"),
        ("Control Remote Desktop", "Controle o Ambiente de Trabalho à distância"),
        ("Transfer file", "Transferir Ficheiro"),
        ("Connect", "Ligar"),
        ("Recent sessions", "Sessões recentes"),
        ("Address book", "Lista de Endereços"),
        ("Confirmation", "Confirmação"),
        ("TCP tunneling", "Túnel TCP"),
        ("Remove", "Remover"),
        ("Refresh random password", "Actualizar palavra-chave"),
        ("Set your own password", "Configure a sua palavra-passe"),
        ("Enable keyboard/mouse", "Activar Teclado/Rato"),
        ("Enable clipboard", "Activar Área de Transferência"),
        ("Enable file transfer", "Activar Transferência de Ficheiros"),
        ("Enable TCP tunneling", "Activar Túnel TCP"),
        ("IP Whitelisting", "Whitelist de IP"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import server config", "Importar Configuração do Servidor"),
        ("Export Server Config", "Exportar Configuração do Servidor"),
        ("Import server configuration successfully", "Configuração do servidor importada com sucesso"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "Configuração do servidor inválida"),
        ("Clipboard is empty", "A área de transferência está vazia"),
        ("Stop service", "Parar serviço"),
        ("Change ID", "Alterar ID"),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Somente os caracteres a-z, A-Z, 0-9 e _ (sublinhado) são permitidos. A primeira letra deve ser a-z, A-Z. Comprimento entre 6 e 16."),
        ("Website", "Website"),
        ("About", "Sobre"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Silenciar"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Entrada de Áudio"),
        ("Enhancements", "Melhorias"),
        ("Hardware Codec", ""),
        ("Adaptive bitrate", ""),
        ("ID Server", "Servidor de ID"),
        ("Relay Server", "Servidor de Relay"),
        ("API Server", "Servidor da API"),
        ("invalid_http", "deve iniciar com http:// ou https://"),
        ("Invalid IP", "IP inválido"),
        ("Invalid format", "Formato inválido"),
        ("server_not_support", "Ainda não suportado pelo servidor"),
        ("Not available", "Indisponível"),
        ("Too frequent", "Muito frequente"),
        ("Cancel", "Cancelar"),
        ("Skip", "Passar"),
        ("Close", "Fechar"),
        ("Retry", "Tentar novamente"),
        ("OK", "Confirmar"),
        ("Password Required", "Palavra-chave Necessária"),
        ("Please enter your password", "Por favor introduza a sua palavra-chave"),
        ("Remember password", "Memorizar palavra-chave"),
        ("Wrong Password", "Palavra-chave inválida"),
        ("Do you want to enter again?", "Deseja tentar novamente??"),
        ("Connection Error", "Erro de Ligação"),
        ("Error", "Erro"),
        ("Reset by the peer", "Reiniciado pelo destino"),
        ("Connecting...", "A Ligar..."),
        ("Connection in progress. Please wait.", "Ligação em progresso. Aguarde por favor."),
        ("Please try 1 minute later", "Por favor tente após 1 minuto"),
        ("Login Error", "Erro de Login"),
        ("Successful", "Sucesso"),
        ("Connected, waiting for image...", "Ligado. A aguardar pela imagem..."),
        ("Name", "Nome"),
        ("Type", "Tipo"),
        ("Modified", "Modificado"),
        ("Size", "Tamanho"),
        ("Show Hidden Files", "Mostrar Ficheiros Ocultos"),
        ("Receive", "Receber"),
        ("Send", "Enviar"),
        ("Refresh File", "Actualizar Ficheiro"),
        ("Local", "Local"),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computador Remoto"),
        ("Local Computer", "Computador Local"),
        ("Confirm Delete", "Confirmar Apagar"),
        ("Delete", "Apagar"),
        ("Properties", "Propriedades"),
        ("Multi Select", "Selecção Múltipla"),
        ("Select All", "Selecionar tudo"),
        ("Unselect All", "Desmarcar todos"),
        ("Empty Directory", "Directório Vazio"),
        ("Not an empty directory", "Directório não está vazio"),
        ("Are you sure you want to delete this file?", "Tem certeza que deseja apagar este ficheiro?"),
        ("Are you sure you want to delete this empty directory?", "Tem certeza que deseja apagar este directório vazio?"),
        ("Are you sure you want to delete the file of this directory?", "Tem certeza que deseja apagar este ficheiro deste directório?"),
        ("Do this for all conflicts", "Fazer isto para todos os conflictos"),
        ("This is irreversible!", "Isto é irreversível!"),
        ("Deleting", "A apagar"),
        ("files", "ficheiros"),
        ("Waiting", "A aguardar"),
        ("Finished", "Completo"),
        ("Speed", "Velocidade"),
        ("Custom Image Quality", "Qualidade Visual Personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de utilizador"),
        ("Unblock user input", "Desbloquear entrada de utilizador"),
        ("Adjust Window", "Ajustar Janela"),
        ("Original", "Original"),
        ("Shrink", "Reduzir"),
        ("Stretch", "Aumentar"),
        ("Scrollbar", ""),
        ("ScrollAuto", ""),
        ("Good image quality", "Qualidade visual boa"),
        ("Balanced", "Equilibrada"),
        ("Optimize reaction time", "Optimizar tempo de reacção"),
        ("Custom", ""),
        ("Show remote cursor", "Mostrar cursor remoto"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Desabilitar área de transferência"),
        ("Lock after session end", "Bloquear após o fim da sessão"),
        ("Insert", "Inserir"),
        ("Insert Lock", "Bloquear Inserir"),
        ("Refresh", "Actualizar"),
        ("ID does not exist", "ID não existente"),
        ("Failed to connect to rendezvous server", "Falha ao ligar ao servidor de rendezvous"),
        ("Please try later", "Por favor tente mais tarde"),
        ("Remote desktop is offline", "Ambiente de trabalho remoto está desligado"),
        ("Key mismatch", "Chaves incompatíveis"),
        ("Timeout", "Tempo esgotado"),
        ("Failed to connect to relay server", "Falha ao ligar ao servidor de relay"),
        ("Failed to connect via rendezvous server", "Falha ao ligar ao servidor de rendezvous"),
        ("Failed to connect via relay server", "Falha ao ligar através do servidor de relay"),
        ("Failed to make direct connection to remote desktop", "Falha ao fazer ligação directa ao desktop remoto"),
        ("Set Password", "Definir palavra-chave"),
        ("OS Password", "Senha do SO"),
        ("install_tip", "Devido ao UAC, o RustDesk não funciona correctamente em alguns casos. Para evitar o UAC, por favor clique no botão abaixo para instalar o RustDesk no sistema."),
        ("Click to upgrade", "Clique para atualizar"),
        ("Click to download", "Clique para carregar"),
        ("Click to update", "Clique para fazer a actualização"),
        ("Configure", "Configurar"),
        ("config_acc", "Para controlar o seu Ambiente de Trabalho remotamente, é preciso conceder ao RustDesk permissões de \"Acessibilidade\"."),
        ("config_screen", "Para aceder ao seu Ambiente de Trabalho remotamente, é preciso conceder ao RustDesk permissões de \"Gravar a Tela\"/"),
        ("Installing ...", "A Instalar ..."),
        ("Install", "Instalar"),
        ("Installation", "Instalação"),
        ("Installation Path", "Caminho da Instalação"),
        ("Create start menu shortcuts", "Criar atalhos no menu iniciar"),
        ("Create desktop icon", "Criar ícone no ambiente de trabalho"),
        ("agreement_tip", "Ao iniciar a instalação, você concorda com o acordo de licença."),
        ("Accept and Install", "Aceitar e Instalar"),
        ("End-user license agreement", "Acordo de licença do utilizador final"),
        ("Generating ...", "A Gerar ..."),
        ("Your installation is lower version.", "A sua instalação é de uma versão anterior."),
        ("not_close_tcp_tip", "Não feche esta janela enquanto estiver a utilizar o túnel"),
        ("Listening ...", "A escuta ..."),
        ("Remote Host", "Host Remoto"),
        ("Remote Port", "Porta Remota"),
        ("Action", "Acção"),
        ("Add", "Adicionar"),
        ("Local Port", "Porta Local"),
        ("Local Address", "Endereço local"),
        ("Change Local Port", "Alterar porta local"),
        ("setup_server_tip", "Para uma ligação mais rápida, por favor configure seu próprio servidor"),
        ("Too short, at least 6 characters.", "Muito curto, pelo menos 6 caracteres."),
        ("The confirmation is not identical.", "A confirmação não é idêntica."),
        ("Permissions", "Permissões"),
        ("Accept", "Aceitar"),
        ("Dismiss", "Dispensar"),
        ("Disconnect", "Desconectar"),
        ("Enable file copy and paste", "Permitir copiar e mover ficheiros"),
        ("Connected", "Ligado"),
        ("Direct and encrypted connection", "Ligação directa e encriptada"),
        ("Relayed and encrypted connection", "Ligação via relay e encriptada"),
        ("Direct and unencrypted connection", "Ligação direta e não encriptada"),
        ("Relayed and unencrypted connection", "Ligação via relay e não encriptada"),
        ("Enter Remote ID", "Introduza o ID Remoto"),
        ("Enter your password", "Introduza a sua palavra-chave"),
        ("Logging in...", "A efectuar Login..."),
        ("Enable RDP session sharing", "Activar partilha de sessão RDP"),
        ("Auto Login", "Login Automático (Somente válido se você activou \"Bloquear após o fim da sessão\")"),
        ("Enable direct IP access", "Activar Acesso IP Directo"),
        ("Rename", "Renomear"),
        ("Space", "Espaço"),
        ("Create desktop shortcut", "Criar Atalho no Ambiente de Trabalho"),
        ("Change Path", "Alterar Caminho"),
        ("Create Folder", "Criar Diretório"),
        ("Please enter the folder name", "Por favor introduza o nome do diretório"),
        ("Fix it", "Reparar"),
        ("Warning", "Aviso"),
        ("Login screen using Wayland is not supported", "Tela de Login com Wayland não é suportada"),
        ("Reboot required", "Reinicialização necessária"),
        ("Unsupported display server", "Servidor de display não suportado"),
        ("x11 expected", "x11 em falha"),
        ("Port", ""),
        ("Settings", "Configurações"),
        ("Username", "Nome de utilizador"),
        ("Invalid port", "Porta inválida"),
        ("Closed manually by the peer", "Fechada manualmente pelo destino"),
        ("Enable remote configuration modification", "Habilitar modificações de configuração remotas"),
        ("Run without install", "Executar sem instalar"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Sempre conectar via relay"),
        ("whitelist_tip", "Somente IPs na whitelist podem me acessar"),
        ("Login", "Login"),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", ""),
        ("Logout", "Sair"),
        ("Tags", "Tags"),
        ("Search ID", "Procurar ID"),
        ("whitelist_sep", "Separado por vírcula, ponto-e-vírgula, espaços ou nova linha"),
        ("Add ID", "Adicionar ID"),
        ("Add Tag", "Adicionar Tag"),
        ("Unselect all tags", "Desselecionar todas as tags"),
        ("Network error", "Erro de rede"),
        ("Username missed", "Nome de utilizador em falta"),
        ("Password missed", "Palavra-chave em falta"),
        ("Wrong credentials", "Nome de utilizador ou palavra-chave incorrectos"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Editar Tag"),
        ("Forget Password", "Esquecer Palavra-chave"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Adicionar aos Favoritos"),
        ("Remove from Favorites", "Remover dos Favoritos"),
        ("Empty", "Vazio"),
        ("Invalid folder name", "Nome de diretório inválido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Socks5/Http(s) Proxy", "Proxy Socks5/Http(s)"),
        ("Discovered", "Descoberto"),
        ("install_daemon_tip", "Para inicialização junto do sistema, deve instalar o serviço de sistema."),
        ("Remote ID", "ID Remoto"),
        ("Paste", "Colar"),
        ("Paste here?", "Colar aqui?"),
        ("Are you sure to close the connection?", "Tem certeza que deseja fechar a ligação?"),
        ("Download new version", "Transferir nova versão"),
        ("Touch mode", "Modo toque"),
        ("Mouse mode", "Modo rato"),
        ("One-Finger Tap", "Toque com um dedo"),
        ("Left Mouse", "Botão esquerdo do rato"),
        ("One-Long Tap", "Um toque longo"),
        ("Two-Finger Tap", "Toque com dois dedos"),
        ("Right Mouse", "Botão direito do rato"),
        ("One-Finger Move", "Mover com um dedo"),
        ("Double Tap & Move", "Toque duplo & mover"),
        ("Mouse Drag", "Arrastar com o rato"),
        ("Three-Finger vertically", "Três dedos verticalmente"),
        ("Mouse Wheel", "Roda do rato"),
        ("Two-Finger Move", "Mover com dois dedos"),
        ("Canvas Move", "Mover Tela"),
        ("Pinch to Zoom", "Clique para ampliar"),
        ("Canvas Zoom", "Zoom na Tela"),
        ("Reset canvas", "Reiniciar tela"),
        ("No permission of file transfer", "Sem permissões de transferência de ficheiro"),
        ("Note", "Nota"),
        ("Connection", "Ligação"),
        ("Share Screen", "Partilhar ecrã"),
        ("Chat", "Conversar"),
        ("Total", "Total"),
        ("items", "itens"),
        ("Selected", "Seleccionado"),
        ("Screen Capture", "Captura de Ecran"),
        ("Input Control", "Controle de Entrada"),
        ("Audio Capture", "Captura de Áudio"),
        ("File Connection", "Ligação de Arquivo"),
        ("Screen Connection", "Ligação de Ecran"),
        ("Do you accept?", "Aceita?"),
        ("Open System Setting", "Abrir Configurações do Sistema"),
        ("How to get Android input permission?", "Como activar a permissão de entrada do Android?"),
        ("android_input_permission_tip1", "Para que um dispositivo remoto controle o seu dispositivo Android via rato ou toque, você precisa permitir que o RustDesk use o serviço \"Acessibilidade\"."),
        ("android_input_permission_tip2", "Por favor vá para a próxima página de configuração do sistema, encontre e entre [Serviços Instalados], ACTIVE o serviço [RustDesk Input]."),
        ("android_new_connection_tip", "Nova requisição de controle recebida, solicita o controle do seu dispositivo atual."),
        ("android_service_will_start_tip", "Activar a Captura de Ecran irá automaticamente inicializar o serviço, permitindo que outros dispositivos solicitem uma ligação deste dispositivo."),
        ("android_stop_service_tip", "Fechar o serviço irá automaticamente fechar todas as ligações estabelecidas."),
        ("android_version_audio_tip", "A versão atual do Android não suporta captura de áudio, por favor actualize para o Android 10 ou maior."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", ""),
        ("Overwrite", "Substituir"),
        ("This file exists, skip or overwrite this file?", "Este ficheiro já existe, ignorar ou substituir este ficheiro?"),
        ("Quit", "Saída"),
        ("Help", "Ajuda"),
        ("Failed", "Falhou"),
        ("Succeeded", "Conseguiu"),
        ("Someone turns on privacy mode, exit", "Alguém activou o modo de privacidade, desligue"),
        ("Unsupported", "Sem suporte"),
        ("Peer denied", "Remoto negado"),
        ("Please install plugins", "Por favor instale plugins"),
        ("Peer exit", "Saída do Remoto"),
        ("Failed to turn off", "Falha ao desligar"),
        ("Turned off", "Desligado"),
        ("Language", "Linguagem"),
        ("Keep RustDesk background service", "Manter o serviço RustDesk em funcionamento"),
        ("Ignore Battery Optimizations", "Ignorar optimizações de Bateria"),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Ligação não autorizada"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", "Utilizar palavra-chave permanente"),
        ("Use both passwords", "Utilizar ambas as palavras-chave"),
        ("Set permanent password", "Definir palavra-chave permanente"),
        ("Enable remote restart", "Activar reiniciar remoto"),
        ("Restart remote device", "Reiniciar Dispositivo Remoto"),
        ("Are you sure you want to restart", "Tem a certeza que pretende reiniciar"),
        ("Restarting remote device", "A reiniciar sistema remoto"),
        ("remote_restarting_tip", ""),
        ("Copied", ""),
        ("Exit Fullscreen", "Sair da tela cheia"),
        ("Fullscreen", "Tela cheia"),
        ("Mobile Actions", "Ações para celular"),
        ("Select Monitor", "Selecionar monitor"),
        ("Control Actions", "Ações de controle"),
        ("Display Settings", "Configurações do visor"),
        ("Ratio", "Razão"),
        ("Image Quality", "Qualidade da imagem"),
        ("Scroll Style", "Estilo de rolagem"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Conexão direta"),
        ("Relay Connection", "Conexão de relé"),
        ("Secure Connection", "Conexão segura"),
        ("Insecure Connection", "Conexão insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptável"),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Light Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable recording session", ""),
        ("Enable LAN discovery", ""),
        ("Deny LAN discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Desconectado"),
        ("Other", "Outro"),
        ("Confirm before closing multiple tabs", "Confirme antes de fechar vários separadores"),
        ("Keyboard Settings", "Configurações do teclado"),
        ("Full Access", "Controlo total"),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requer Ubuntu 21.04 ou versão superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requer uma versão superior da distribuição linux. Por favor, tente o desktop X11 ou mude seu sistema operacional."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Por favor, selecione a tela a ser compartilhada (operar no lado do peer)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
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
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
        ("Follow remote cursor", ""),
        ("Follow remote window focus", ""),
        ("default_proxy_tip", ""),
        ("no_audio_input_device_tip", ""),
        ("Incoming", ""),
        ("Outgoing", ""),
        ("Clear wayland screen selection", ""),
        ("clear_wayland_screen_selection_tip", ""),
        ("confirm_clear_wayland_screen_selection_tip", ""),
    ].iter().cloned().collect();
}
