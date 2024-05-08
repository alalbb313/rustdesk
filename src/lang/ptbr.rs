lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Seu Computador"),
        ("desk_tip", "Seu computador pode ser acessado com este ID e senha."),
        ("Password", "Senha"),
        ("Ready", "Pronto"),
        ("Established", "Estabelecido"),
        ("connecting_status", "Conectando à rede do RustDesk..."),
        ("Enable service", "Habilitar Serviço"),
        ("Start service", "Iniciar Serviço"),
        ("Service is running", "Serviço está em execução"),
        ("Service is not running", "Serviço não está em execução"),
        ("not_ready_status", "Não está pronto. Por favor verifique sua conexão"),
        ("Control Remote Desktop", "Controle um Computador Remoto"),
        ("Transfer file", "Transferir Arquivos"),
        ("Connect", "Conectar"),
        ("Recent sessions", "Sessões Recentes"),
        ("Address book", "Lista de Endereços"),
        ("Confirmation", "Confirmação"),
        ("TCP tunneling", "Tunelamento TCP"),
        ("Remove", "Remover"),
        ("Refresh random password", "Atualizar senha aleatória"),
        ("Set your own password", "Configure sua própria senha"),
        ("Enable keyboard/mouse", "Habilitar teclado/mouse"),
        ("Enable clipboard", "Habilitar Área de Transferência"),
        ("Enable file transfer", "Habilitar Transferência de Arquivos"),
        ("Enable TCP tunneling", "Habilitar Tunelamento TCP"),
        ("IP Whitelisting", "Lista de IPs Confiáveis"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import server config", "Importar Configuração do Servidor"),
        ("Export Server Config", "Exportar Configuração do Servidor"),
        ("Import server configuration successfully", "Configuração do servidor importada com sucesso"),
        ("Export server configuration successfully", "Configuração do servidor exportada com sucesso"),
        ("Invalid server configuration", "Configuração do servidor inválida"),
        ("Clipboard is empty", "A área de transferência está vazia"),
        ("Stop service", "Parar serviço"),
        ("Change ID", "Alterar ID"),
        ("Your new ID", "Seu novo ID"),
        ("length %min% to %max%", "tamanho %min% para %max%"),
        ("starts with a letter", "começa com uma letra"),
        ("allowed characters", "caracteres permitidos"),
        ("id_change_tip", "Somente os caracteres a-z, A-Z, 0-9 e _ (sublinhado) são permitidos. A primeira letra deve ser a-z, A-Z. Comprimento entre 6 e 16."),
        ("Website", "Website"),
        ("About", "Sobre"),
        ("Slogan_tip", "Feito de coração neste mundo caótico!"),
        ("Privacy Statement", "Declaração de Privacidade"),
        ("Mute", "Desativar som"),
        ("Build Date", "Data da Build"),
        ("Version", "Versão"),
        ("Home", "Início"),
        ("Audio Input", "Entrada de Áudio"),
        ("Enhancements", "Melhorias"),
        ("Hardware Codec", "Codec de hardware"),
        ("Adaptive bitrate", "Taxa de bits adaptável"),
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
        ("Skip", "Pular"),
        ("Close", "Fechar"),
        ("Retry", "Tentar novamente"),
        ("OK", "OK"),
        ("Password Required", "Senha necessária"),
        ("Please enter your password", "Por favor informe sua senha"),
        ("Remember password", "Lembrar senha"),
        ("Wrong Password", "Senha incorreta"),
        ("Do you want to enter again?", "Você deseja conectar novamente?"),
        ("Connection Error", "Erro de conexão"),
        ("Error", "Erro"),
        ("Reset by the peer", "Reiniciado pelo parceiro"),
        ("Connecting...", "Conectando..."),
        ("Connection in progress. Please wait.", "Conexão em progresso. Aguarde por favor."),
        ("Please try 1 minute later", "Por favor tente após 1 minuto"),
        ("Login Error", "Erro de login"),
        ("Successful", "Sucesso"),
        ("Connected, waiting for image...", "Conectado. Aguardando pela imagem..."),
        ("Name", "Nome"),
        ("Type", "Tipo"),
        ("Modified", "Modificado"),
        ("Size", "Tamanho"),
        ("Show Hidden Files", "Mostrar Arquivos Ocultos"),
        ("Receive", "Receber"),
        ("Send", "Enviar"),
        ("Refresh File", "Atualizar Arquivo"),
        ("Local", "Local"),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computador Remoto"),
        ("Local Computer", "Computador Local"),
        ("Confirm Delete", "Confirmar Apagar"),
        ("Delete", "Apagar"),
        ("Properties", "Propriedades"),
        ("Multi Select", "Seleção múltipla"),
        ("Select All", "Selecionar tudo"),
        ("Unselect All", "Desmarcar tudo"),
        ("Empty Directory", "Diretório vazio"),
        ("Not an empty directory", "Diretório não está vazio"),
        ("Are you sure you want to delete this file?", "Tem certeza que deseja apagar este arquivo?"),
        ("Are you sure you want to delete this empty directory?", "Tem certeza que deseja apagar este diretório vazio?"),
        ("Are you sure you want to delete the file of this directory?", "Tem certeza que deseja apagar este arquivo deste diretório?"),
        ("Do this for all conflicts", "Fazer isto para todos os conflitos"),
        ("This is irreversible!", "Isso é irreversível!"),
        ("Deleting", "Apagando"),
        ("files", "arquivos"),
        ("Waiting", "Aguardando"),
        ("Finished", "Completo"),
        ("Speed", "Velocidade"),
        ("Custom Image Quality", "Qualidade Visual Personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de usuário"),
        ("Unblock user input", "Desbloquear entrada de usuário"),
        ("Adjust Window", "Ajustar Janela"),
        ("Original", "Original"),
        ("Shrink", "Reduzir"),
        ("Stretch", "Aumentar"),
        ("Scrollbar", "Barra de rolagem"),
        ("ScrollAuto", "Rolagem automática"),
        ("Good image quality", "Qualidade visual boa"),
        ("Balanced", "Balanceada"),
        ("Optimize reaction time", "Otimizar tempo de resposta"),
        ("Custom", "Personalizado"),
        ("Show remote cursor", "Mostrar cursor remoto"),
        ("Show quality monitor", "Exibir monitor de qualidade"),
        ("Disable clipboard", "Desabilitar área de transferência"),
        ("Lock after session end", "Bloquear após o fim da sessão"),
        ("Insert", "Inserir"),
        ("Insert Lock", "Bloquear computador"),
        ("Refresh", "Atualizar"),
        ("ID does not exist", "ID não existe"),
        ("Failed to connect to rendezvous server", "Falha ao conectar ao servidor de rendezvous"),
        ("Please try later", "Por favor tente mais tarde"),
        ("Remote desktop is offline", "O computador remoto está offline"),
        ("Key mismatch", "Chaves incompatíveis"),
        ("Timeout", "Tempo esgotado"),
        ("Failed to connect to relay server", "Falha ao conectar ao servidor de relay"),
        ("Failed to connect via rendezvous server", "Falha ao conectar ao servidor de rendezvous"),
        ("Failed to connect via relay server", "Falha ao conectar através do servidor de relay"),
        ("Failed to make direct connection to remote desktop", "Falha ao fazer conexão direta ao desktop remoto"),
        ("Set Password", "Definir Senha"),
        ("OS Password", "Senha do SO"),
        ("install_tip", "Devido ao UAC, o RustDesk não funciona corretamente como o lado remoto em alguns casos. Para evitar o UAC, por favor clique no botão abaixo para instalar o RustDesk no sistema."),
        ("Click to upgrade", "Clique para fazer o upgrade"),
        ("Click to download", "Clique para baixar"),
        ("Click to update", "Clique para fazer o update"),
        ("Configure", "Configurar"),
        ("config_acc", "Para controlar seu computador remotamente, você precisa conceder ao RustDesk permissões de \"Acessibilidade\"."),
        ("config_screen", "Para acessar seu computador remotamente, você precisa conceder ao RustDesk permissões de \"Gravar a Tela\"/"),
        ("Installing ...", "Instalando ..."),
        ("Install", "Instalar"),
        ("Installation", "Instalação"),
        ("Installation Path", "Caminho da Instalação"),
        ("Create start menu shortcuts", "Criar atalhos no Menu Iniciar"),
        ("Create desktop icon", "Criar ícone na Área de Trabalho"),
        ("agreement_tip", "Ao iniciar a instalação, você concorda com o acordo de licença."),
        ("Accept and Install", "Aceitar e Instalar"),
        ("End-user license agreement", "Acordo de licença do usuário final"),
        ("Generating ...", "Gerando ..."),
        ("Your installation is lower version.", "Sua instalação é de uma versão menor."),
        ("not_close_tcp_tip", "Não feche esta janela enquanto estiver utilizando o túnel"),
        ("Listening ...", "Escutando ..."),
        ("Remote Host", "Host Remoto"),
        ("Remote Port", "Porta Remota"),
        ("Action", "Ação"),
        ("Add", "Adicionar"),
        ("Local Port", "Porta Local"),
        ("Local Address", "Endereço Local"),
        ("Change Local Port", "Alterar Porta Local"),
        ("setup_server_tip", "Para uma conexão mais rápida, por favor configure seu próprio servidor"),
        ("Too short, at least 6 characters.", "Muito curto, pelo menos 6 caracteres."),
        ("The confirmation is not identical.", "A confirmação não é idêntica."),
        ("Permissions", "Permissões"),
        ("Accept", "Aceitar"),
        ("Dismiss", "Dispensar"),
        ("Disconnect", "Desconectar"),
        ("Enable file copy and paste", "Permitir copiar e colar arquivos"),
        ("Connected", "Conectado"),
        ("Direct and encrypted connection", "Conexão direta e criptografada"),
        ("Relayed and encrypted connection", "Conexão via relay e criptografada"),
        ("Direct and unencrypted connection", "Conexão direta e não criptografada"),
        ("Relayed and unencrypted connection", "Conexão via relay e não criptografada"),
        ("Enter Remote ID", "Informe o ID Remoto"),
        ("Enter your password", "Informe sua senha"),
        ("Logging in...", "Fazendo Login..."),
        ("Enable RDP session sharing", "Habilitar compartilhamento de sessão RDP"),
        ("Auto Login", "Login Automático (Somente válido se você habilitou \"Bloquear após o fim da sessão\")"),
        ("Enable direct IP access", "Habilitar Acesso IP Direto"),
        ("Rename", "Renomear"),
        ("Space", "Espaço"),
        ("Create desktop shortcut", "Criar Atalho na Área de Trabalho"),
        ("Change Path", "Alterar Caminho"),
        ("Create Folder", "Criar Diretório"),
        ("Please enter the folder name", "Por favor informe o nome do diretório"),
        ("Fix it", "Corrigir"),
        ("Warning", "Aviso"),
        ("Login screen using Wayland is not supported", "Tela de Login utilizando Wayland não é suportada"),
        ("Reboot required", "Reinicialização necessária"),
        ("Unsupported display server", "Servidor de display não suportado"),
        ("x11 expected", "x11 esperado"),
        ("Port", "Porta"),
        ("Settings", "Configurações"),
        ("Username", "Nome de usuário"),
        ("Invalid port", "Porta inválida"),
        ("Closed manually by the peer", "Fechada manualmente pelo parceiro"),
        ("Enable remote configuration modification", "Habilitar modificações de configuração remotas"),
        ("Run without install", "Executar sem instalar"),
        ("Connect via relay", "Conectar via relay"),
        ("Always connect via relay", "Sempre conectar via relay"),
        ("whitelist_tip", "Somente IPs confiáveis podem me acessar"),
        ("Login", "Login"),
        ("Verify", "Verificar"),
        ("Remember me", "Lembrar de mim"),
        ("Trust this device", "Confiar neste dispositivo"),
        ("Verification code", "Código de verificação"),
        ("verification_tip", "Um novo dispositivo foi detectado e um código de verificação foi enviado para o endereço de e-mail registrado, insira o código de verificação para continuar o login."),
        ("Logout", "Sair"),
        ("Tags", "Tags"),
        ("Search ID", "Pesquisar ID"),
        ("whitelist_sep", "Separado por vírcula, ponto-e-vírgula, espaços ou nova linha"),
        ("Add ID", "Adicionar ID"),
        ("Add Tag", "Adicionar Tag"),
        ("Unselect all tags", "Desmarcar todas as tags"),
        ("Network error", "Erro de rede"),
        ("Username missed", "Nome de usuário requerido"),
        ("Password missed", "Senha requerida"),
        ("Wrong credentials", "Nome de usuário ou senha incorretos"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Editar Tag"),
        ("Forget Password", "Esquecer Senha"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Adicionar aos Favoritos"),
        ("Remove from Favorites", "Remover dos Favoritos"),
        ("Empty", "Vazio"),
        ("Invalid folder name", "Nome de diretório inválido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Socks5/Http(s) Proxy", "Proxy Socks5/Http(s)"),
        ("Discovered", "Descoberto"),
        ("install_daemon_tip", "Para inicialização junto ao sistema, você deve instalar o serviço de sistema."),
        ("Remote ID", "ID Remoto"),
        ("Paste", "Colar"),
        ("Paste here?", "Colar aqui?"),
        ("Are you sure to close the connection?", "Tem certeza que deseja fechar a conexão?"),
        ("Download new version", "Baixar nova versão"),
        ("Touch mode", "Modo toque"),
        ("Mouse mode", "Modo mouse"),
        ("One-Finger Tap", "Toque com um dedo"),
        ("Left Mouse", "Botão esquerdo do mouse"),
        ("One-Long Tap", "Um toque longo"),
        ("Two-Finger Tap", "Toque com dois dedos"),
        ("Right Mouse", "Botão direito do mouse"),
        ("One-Finger Move", "Mover com um dedo"),
        ("Double Tap & Move", "Toque duplo & mover"),
        ("Mouse Drag", "Arrastar com o mouse"),
        ("Three-Finger vertically", "Três dedos verticalmente"),
        ("Mouse Wheel", "Roda do Mouse"),
        ("Two-Finger Move", "Mover com dois dedos"),
        ("Canvas Move", "Mover Tela"),
        ("Pinch to Zoom", "Pinçar para Zoom"),
        ("Canvas Zoom", "Zoom na tela"),
        ("Reset canvas", "Reiniciar tela"),
        ("No permission of file transfer", "Sem permissão para transferência de arquivo"),
        ("Note", "Nota"),
        ("Connection", "Conexão"),
        ("Share Screen", "Compartilhar Tela"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "itens"),
        ("Selected", "Selecionado"),
        ("Screen Capture", "Captura de Tela"),
        ("Input Control", "Controle de Entrada"),
        ("Audio Capture", "Captura de Áudio"),
        ("File Connection", "Conexão de Arquivo"),
        ("Screen Connection", "Conexão de Tela"),
        ("Do you accept?", "Você aceita?"),
        ("Open System Setting", "Abrir Configurações do Sistema"),
        ("How to get Android input permission?", "Como habilitar a permissão de entrada do Android?"),
        ("android_input_permission_tip1", "Para que um dispositivo remoto controle seu dispositivo Android via mouse ou toque, você precisa permitir que o RustDesk use o serviço \"Acessibilidade\"."),
        ("android_input_permission_tip2", "Por favor vá para a próxima página de configuração do sistema, encontre e entre [Serviços Instalados], HABILITE o serviço [RustDesk Input]."),
        ("android_new_connection_tip", "Nova requisição de controle recebida, solicita o controle de seu dispositivo atual."),
        ("android_service_will_start_tip", "Habilitar a Captura de Tela irá automaticamente inicalizar o serviço, permitindo que outros dispositivos solicitem uma conexão deste dispositivo."),
        ("android_stop_service_tip", "Fechar o serviço irá automaticamente fechar todas as conexões estabelecidas."),
        ("android_version_audio_tip", "A versão atual do Android não suporta captura de áudio, por favor atualize para o Android 10 ou superior."),
        ("android_start_service_tip", "Toque em [Iniciar serviço] ou habilite a permissão [Captura de tela] para iniciar o serviço de compartilhamento de tela."),
        ("android_permission_may_not_change_tip", "As permissões para conexões estabelecidas podem não ser alteradas instantaneamente até que seja reconectado."),
        ("Account", "Conta"),
        ("Overwrite", "Substituir"),
        ("This file exists, skip or overwrite this file?", "Este arquivo existe, pular ou substituir este arquivo?"),
        ("Quit", "Sair"),
        ("Help", "Ajuda"),
        ("Failed", "Falhou"),
        ("Succeeded", "Sucesso"),
        ("Someone turns on privacy mode, exit", "Alguém habilitou o modo de privacidade, sair"),
        ("Unsupported", "Não suportado"),
        ("Peer denied", "Parceiro negou"),
        ("Please install plugins", "Por favor instale plugins"),
        ("Peer exit", "Parceiro saiu"),
        ("Failed to turn off", "Falha ao desligar"),
        ("Turned off", "Desligado"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Manter o serviço do RustDesk executando em segundo plano"),
        ("Ignore Battery Optimizations", "Ignorar otimizações de bateria"),
        ("android_open_battery_optimizations_tip", "Abrir otimizações de bateria"),
        ("Start on boot", "Iniciar na Inicialização"),
        ("Start the screen sharing service on boot, requires special permissions", "Inicie o serviço de compartilhamento de tela na inicialização, requer permissões especiais"),
        ("Connection not allowed", "Conexão não permitida"),
        ("Legacy mode", "Modo legado"),
        ("Map mode", "Modo mapa"),
        ("Translate mode", "Modo traduzido"),
        ("Use permanent password", "Utilizar senha permanente"),
        ("Use both passwords", "Utilizar ambas as senhas"),
        ("Set permanent password", "Configurar senha permanente"),
        ("Enable remote restart", "Habilitar Reinicialização Remota"),
        ("Restart remote device", "Reiniciar Dispositivo Remoto"),
        ("Are you sure you want to restart", "Você tem certeza que deseja reiniciar?"),
        ("Restarting remote device", "Reiniciando dispositivo remoto"),
        ("remote_restarting_tip", "O dispositivo remoto está reiniciando, feche esta caixa de mensagem e reconecte com a senha permanente depois de um tempo"),
        ("Copied", "Copiado"),
        ("Exit Fullscreen", "Sair da Tela Cheia"),
        ("Fullscreen", "Tela Cheia"),
        ("Mobile Actions", "Ações móveis"),
        ("Select Monitor", "Selecionar monitor"),
        ("Control Actions", "Controlar ações"),
        ("Display Settings", "Configurações de exibição"),
        ("Ratio", "Proporção"),
        ("Image Quality", "Qualidade de Imagem"),
        ("Scroll Style", "Estilo de Rolagem"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Conexão Direta"),
        ("Relay Connection", "Conexão via Relay"),
        ("Secure Connection", "Conexão Segura"),
        ("Insecure Connection", "Conexão Insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptada"),
        ("General", "Geral"),
        ("Security", "Segurança"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Escuro"),
        ("Light Theme", "Tema Claro"),
        ("Dark", "Escuro"),
        ("Light", "Claro"),
        ("Follow System", "Seguir sistema"),
        ("Enable hardware codec", "Habilitar codec de hardware"),
        ("Unlock Security Settings", "Desbloquear configurações de segurança"),
        ("Enable audio", "Habilitar áudio"),
        ("Unlock Network Settings", "Desbloquear configurações de rede"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Acesso direto por IP"),
        ("Proxy", "Proxy"),
        ("Apply", "Aplicar"),
        ("Disconnect all devices?", "Desconectar todos os dispositivos?"),
        ("Clear", "Limpar"),
        ("Audio Input Device", "Dispositivo de entrada de áudio"),
        ("Use IP Whitelisting", "Utilizar lista de IPs confiáveis"),
        ("Network", "Rede"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "Gravando"),
        ("Directory", "Diretório"),
        ("Automatically record incoming sessions", "Gravar automaticamente sessões de entrada"),
        ("Change", "Alterar"),
        ("Start session recording", "Iniciar gravação da sessão"),
        ("Stop session recording", "Parar gravação da sessão"),
        ("Enable recording session", "Habilitar gravação da sessão"),
        ("Enable LAN discovery", "Habilitar descoberta da LAN"),
        ("Deny LAN discovery", "Negar descoberta da LAN"),
        ("Write a message", "Escrever uma mensagem"),
        ("Prompt", "Prompt de comando"),
        ("Please wait for confirmation of UAC...", "Favor aguardar a confirmação do UAC..."),
        ("elevated_foreground_window_tip", "A janela atual da área de trabalho remota requer privilégios mais altos para operar, portanto, não é possível usar o mouse e o teclado temporariamente. Você pode solicitar ao usuário remoto para minimizar a janela atual ou clicar no botão de elevação na janela de gerenciamento de conexão. Para evitar esse problema, é recomendável instalar o software no dispositivo remoto."),
        ("Disconnected", "Desconectado"),
        ("Other", "Outro"),
        ("Confirm before closing multiple tabs", "Confirmar antes de fechar múltiplas abas"),
        ("Keyboard Settings", "Configurações de teclado"),
        ("Full Access", "Acesso completo"),
        ("Screen Share", "Compartilhamento de tela"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requer Ubuntu 21.04 ou versão superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requer uma versão superior da distribuição linux. Por favor, tente o desktop X11 ou mude seu sistema operacional."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Por favor, selecione a tela a ser compartilhada (operar no lado do parceiro)."),
        ("Show RustDesk", "Exibir RustDesk"),
        ("This PC", "Este Computador"),
        ("or", "ou"),
        ("Continue with", "Continuar com"),
        ("Elevate", "Elevar"),
        ("Zoom cursor", "Aumentar cursor"),
        ("Accept sessions via password", "Aceitar sessões via senha"),
        ("Accept sessions via click", "Aceitar sessões via clique"),
        ("Accept sessions via both", "Aceitar sessões de ambos os modos"),
        ("Please wait for the remote side to accept your session request...", "Por favor aguarde enquanto o cliente remoto aceita seu pedido de sessão..."),
        ("One-time Password", "Senha de uso único"),
        ("Use one-time password", "Usar senha de uso único"),
        ("One-time password length", "Comprimento da senha de uso único"),
        ("Request access to your device", "Solicitar acesso ao seu dispositivo"),
        ("Hide connection management window", "Ocultar janela de gerenciamento de conexão"),
        ("hide_cm_tip", "Permitir ocultação somente se aceitar sessões via senha e usar senha permanente"),
        ("wayland_experiment_tip", "O suporte ao Wayland está em estágio experimental, use o X11 se precisar de acesso autônomo."),
        ("Right click to select tabs", "Clique com o botão direito para selecionar as guias"),
        ("Skipped", "Ignorado"),
        ("Add to address book", "Adicionar ao livro de endereços"),
        ("Group", "Grupo"),
        ("Search", "Buscar"),
        ("Closed manually by web console", "Fechado manualmente pelo console da web"),
        ("Local keyboard type", "Tipo de teclado local"),
        ("Select local keyboard type", "Selecione o tipo de teclado local"),
        ("software_render_tip", "Se você tiver uma placa gráfica Nvidia e a janela remota fechar imediatamente após a conexão, instalar o driver nouveau e optar por usar a renderização de software pode ajudar. É necessário reiniciar o software."),
        ("Always use software rendering", "Sempre utilizar renderização de software"),
        ("config_input", "Para controlar a área de trabalho remota com teclado, você precisa conceder a permissão \"Monitoramento de entrada\" do RustDesk."),
        ("config_microphone", "Para falar remotamente, você precisa conceder a permissão \"Gravar áudio\" do RustDesk."),
        ("request_elevation_tip", "Você também pode solicitar elevação se houver alguém do lado remoto."),
        ("Wait", "Espera"),
        ("Elevation Error", "Erro de Elevação"),
        ("Ask the remote user for authentication", "Peça autenticação ao usuário remoto"),
        ("Choose this if the remote account is administrator", "Escolha isto se a conta remota for administrador"),
        ("Transmit the username and password of administrator", "Transmita o nome de usuário e a senha do administrador"),
        ("still_click_uac_tip", "Ainda requer que o usuário remoto clique em OK na janela UAC da execução do RustDesk."),
        ("Request Elevation", "Pedir Elevação"),
        ("wait_accept_uac_tip", "Aguarde até que o usuário remoto aceite a caixa de diálogo do UAC."),
        ("Elevate successfully", "Elevado com sucesso"),
        ("uppercase", "maiúsculo"),
        ("lowercase", "minúsculo"),
        ("digit", "dígito"),
        ("special character", "caractere especial"),
        ("length>=8", "tamanho>=8"),
        ("Weak", "Fraco"),
        ("Medium", "Médio"),
        ("Strong", "Forte"),
        ("Switch Sides", "Trocar de Lado"),
        ("Please confirm if you want to share your desktop?", "Por favor, confirme se você deseja compartilhar sua área de trabalho?"),
        ("Display", "Display"),
        ("Default View Style", "Estilo de Visualização Padrão"),
        ("Default Scroll Style", "Estilo de Rolagem Padrão"),
        ("Default Image Quality", "Qualidade de Imagem Padrão"),
        ("Default Codec", "Codec Padrão"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Automático"),
        ("Other Default Options", "Outras Opções Padrão"),
        ("Voice call", "Chamada de voz"),
        ("Text chat", "Chat de texto"),
        ("Stop voice call", "Parar chamada de voz"),
        ("relay_hint_tip", "Pode não ser possível conectar diretamente, você pode tentar conectar via relay. \nAlém disso, se você quiser usar o relay em sua primeira tentativa, pode adicionar o sufixo \"/r\" ao ID ou selecionar a opção \"Sempre conectar via relay\" no cartão do parceiro."),
        ("Reconnect", "Reconectar"),
        ("Codec", "Codec"),
        ("Resolution", "Resolução"),
        ("No transfers in progress", "Nenhuma transferência em andamento"),
        ("Set one-time password length", "Definir comprimento de senha descartável"),
        ("RDP Settings", "Configurações RDP"),
        ("Sort by", "Ordenar por"),
        ("New Connection", "Nova Conexão"),
        ("Restore", "Restaurar"),
        ("Minimize", "Minimizar"),
        ("Maximize", "Maximizar"),
        ("Your Device", "Seu Dispositivo"),
        ("empty_recent_tip", "Ops, não há sessões recentes!\nHora de planejar uma nova."),
        ("empty_favorite_tip", "Ainda não há parceiros favoritos?\nVamos encontrar alguém para se conectar e adicioná-lo aos seus favoritos!"),
        ("empty_lan_tip", "Ah não, parece que ainda não descobrimos nenhum parceiro."),
        ("empty_address_book_tip", "Oh céus, parece que atualmente não há parceiros listados em seu catálogo de endereços."),
        ("eg: admin", "ex. admin"),
        ("Empty Username", "Nome de Usuário vazio"),
        ("Empty Password", "Senha Vazia"),
        ("Me", "Eu"),
        ("identical_file_tip", "Este arquivo é idêntico ao do parceiro."),
        ("show_monitors_tip", "Mostrar monitores na barra de ferramentas"),
        ("View Mode", "Modo de Visualização"),
        ("login_linux_tip", "Você precisa fazer login na conta Linux remota para habilitar uma sessão de desktop X"),
        ("verify_rustdesk_password_tip", "Verifique a senha do RustDesk"),
        ("remember_account_tip", "Lembrar desta conta"),
        ("os_account_desk_tip", "Esta conta é usada para fazer login no Sistema Operacional remoto e habilitar a sessão da área de trabalho em headless"),
        ("OS Account", "Conta do Sistema Operacional"),
        ("another_user_login_title_tip", "Outro usuário já está logado"),
        ("another_user_login_text_tip", "Desconectar"),
        ("xorg_not_found_title_tip", "Xorg não encontrado"),
        ("xorg_not_found_text_tip", "Por favor, instale o Xorg"),
        ("no_desktop_title_tip", "Nenhuma área de trabalho está disponível"),
        ("no_desktop_text_tip", "Por favor, instale a área de trabalho do GNOME"),
        ("No need to elevate", "Não há necessidade de elevar"),
        ("System Sound", "Som do Sistema"),
        ("Default", "Padrão"),
        ("New RDP", "Novo RDP"),
        ("Fingerprint", "Impressão Digital"),
        ("Copy Fingerprint", "Copiar Impressão Digital"),
        ("no fingerprints", "sem Impressões Digitais"),
        ("Select a peer", "Selecione um parceiro"),
        ("Select peers", "Selecione parceiros"),
        ("Plugins", "Plugins"),
        ("Uninstall", "Desinstalar"),
        ("Update", "Atualizar"),
        ("Enable", "Habilitar"),
        ("Disable", "Desabilitar"),
        ("Options", "Opções"),
        ("resolution_original_tip", "Resolução original"),
        ("resolution_fit_local_tip", "Adequar a resolução local"),
        ("resolution_custom_tip", "Customizar resolução"),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", "Sair"),
        ("Open", "Abrir"),
        ("logout_tip", ""),
        ("Service", "Serviço"),
        ("Start", "Iniciar"),
        ("Stop", "Parar"),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", "Sincronizar com sessões recentes"),
        ("Sort tags", "Classificar tags"),
        ("Open connection in new tab", "Abrir conexão em uma nova aba"),
        ("Move tab to new window", "Mover aba para uma nova janela"),
        ("Can not be empty", "Não pode estar vazio"),
        ("Already exists", ""),
        ("Change Password", "Alterar senha"),
        ("Refresh Password", "Atualizar senha"),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", "Alterar cor"),
        ("Primary Color", "Cor principal"),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", "Eu concordo"),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", "Remover papel de parede durante sessão remota"),
        ("Test", "Teste"),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", "Alterar visualização"),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", "Lista"),
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
        ("Please select the session you want to connect to", "Por favor, selecione a sessão que você deseja se conectar"),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", "Definir senha compartilhada"),
        ("Exist in", ""),
        ("Read-only", "Apenas leitura"),
        ("Read/Write", "Leitura/escrita"),
        ("Full Control", "Controle total"),
        ("share_warning_tip", ""),
        ("Everyone", "Todos"),
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
