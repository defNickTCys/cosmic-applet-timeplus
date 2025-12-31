# Time Plus - Applet COSMIC

**Um applet rico em recursos para o [COSMIC Desktop](https://github.com/pop-os/cosmic-epoch)** que estende a funcionalidade padrão de hora/data/calendário com informações meteorológicas integradas e timer pomodoro.

<p align="center">
  <img src="https://img.shields.io/badge/COSMIC-Desktop-orange?style=for-the-badge" alt="COSMIC Desktop"/>
  <img src="https://img.shields.io/badge/Licença-GPL--3.0-blue?style=for-the-badge" alt="Licença GPL-3.0"/>
  <img src="https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust" alt="Rust 2021"/>
</p>

[🇺🇸 Read in English](README.md)

---

## ✨ Recursos

### 📅 Calendário (Padrão do Sistema)
- Grade de calendário completa com localização adequada
- Navegação por meses
- Destaque do dia atual
- Corresponde exatamente ao applet de hora padrão do COSMIC

### 🌤️ Integração Meteorológica *(Em Breve)*
- Exibição do clima atual
- Temperatura e condições
- Previsões baseadas em localização
- Coordenadas configuráveis

### ⏱️ Timer Pomodoro *(Em Breve)*
- Intervalos de trabalho/pausa personalizáveis
- Notificações na área de trabalho ao concluir
- Presets rápidos (5min, 25min, etc.)
- Estado persistente entre sessões

---

## 🚀 Instalação

### Pré-requisitos
- Ambiente COSMIC Desktop
- Rust toolchain (1.70+)
- Dependências do libcosmic

### A Partir do Código-fonte

```bash
# Clone o repositório
git clone https://github.com/SEU_USUARIO/cosmic-applet-timeplus
cd cosmic-applet-timeplus

# Compile e instale
cargo install --path .

# Reinicie o painel COSMIC
killall cosmic-panel
```

### Adicionando ao Painel

1. Abra as **Configurações do COSMIC**
2. Navegue até **Painel** → **Miniaplicativos**
3. Encontre **"Time Plus"** na lista
4. Clique em **Adicionar**

O applet aparecerá no seu painel!

---

## ⚙️ Configuração

As configurações são armazenadas em:
```
~/.config/cosmic/com.system76.CosmicAppletTimePlus/v1/
```

### Configurações Atuais
- `show_date_in_top_panel`: Mostrar data junto com hora (padrão: `true`)
- `military_time`: Auto-detectado do locale do sistema
- `show_seconds`: Mostrar segundos na exibição de hora (padrão: `false`)
- `first_day_of_week`: Dia inicial do calendário (0=Domingo, 1=Segunda)

---

## 🛠️ Desenvolvimento

### Configuração Rápida

```bash
cd cosmic-applet-timeplus

# Use o script dev para testes rápidos
./dev.sh run    # Compila, instala e recarrega o painel
./dev.sh build  # Apenas compila
./dev.sh reload # Apenas reinicia o painel
```

### Estrutura do Projeto

```
cosmic-applet-timeplus/
├── src/
│   ├── main.rs       # Ponto de entrada
│   ├── lib.rs        # Declarações de módulos
│   ├── window.rs     # Lógica principal do applet
│   ├── config.rs     # Structs de configuração
│   ├── localize.rs   # Sistema i18n
│   ├── time.rs       # Helpers do calendário
│   ├── weather.rs    # Módulo de clima (WIP)
│   └── timer.rs      # Módulo de timer (WIP)
├── i18n/             # Traduções (61 idiomas)
├── data/             # Arquivos desktop
└── dev.sh            # Script helper de desenvolvimento
```

### Adicionando Recursos

O applet é construído sobre a base do `cosmic-applet-time` oficial, garantindo compatibilidade e seguindo padrões de design do COSMIC.

**Para estender:**
1. Adicione novos módulos em `src/`
2. Atualize o enum `Message` em `window.rs`
3. Implemente funções de visualização
4. Adicione traduções em `i18n/`

---

## 🌍 Localização

Time Plus suporta **61 idiomas** prontos para uso, usando o mesmo sistema de localização do applet de hora oficial do COSMIC.

As traduções estão em formato Fluent (arquivos `.ftl`) sob `i18n/`.

Para adicionar ou atualizar traduções:
```bash
# Edite o arquivo do idioma apropriado
nano i18n/pt-BR/cosmic_applet_timeplus.ftl

# Recompile e teste
./dev.sh run
```

---

## 📝 Roadmap

### Fase 1: Fundação ✅
- [x] Fork do cosmic-applet-time
- [x] Estrutura adequada do projeto
- [x] Sistema de build e dependências
- [x] Integração com desktop
- [x] Exibição no painel com auto-locale

### Fase 2: Sistema de Abas 🚧
- [ ] Implementar abas segmentadas (Calendário | Clima | Timer)
- [ ] Extrair calendário para visualização dedicada
- [ ] Garantir altura consistente entre abas

### Fase 3: Módulo de Clima 📍
- [ ] Integração com API OpenWeatherMap
- [ ] Configuração de localização
- [ ] Exibição de clima no popup
- [ ] Mini widget de clima no painel

### Fase 4: Módulo de Timer ⏱️
- [ ] Lógica de timer de contagem regressiva
- [ ] Gerenciamento de presets
- [ ] Notificações no desktop
- [ ] Mini widget de timer no painel

### Fase 5: Refinamento 💎
- [ ] Interface de configurações
- [ ] Atalhos de teclado
- [ ] Melhorias de acessibilidade
- [ ] Otimização de performance

---

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Faça um fork do repositório
2. Crie uma branch de feature (`git checkout -b feature/recurso-incrivel`)
3. Commit suas mudanças (`git commit -m 'Adiciona recurso incrível'`)
4. Push para a branch (`git push origin feature/recurso-incrivel`)
5. Abra um Pull Request

### Estilo de Código
- Siga a formatação padrão do Rust (`cargo fmt`)
- Execute verificações do clippy (`cargo clippy`)
- Garanta que builds passem (`cargo build --release`)
- Teste em ambiente COSMIC real

---

## 📜 Licença

Este projeto está licenciado sob a **GNU General Public License v3.0** - veja o arquivo [LICENSE](LICENSE) para detalhes.

Baseado no [cosmic-applet-time](https://github.com/pop-os/cosmic-applets) da System76.

---

## 🙏 Agradecimentos

- **System76** pelo COSMIC Desktop e o applet de hora base
- Time **Pop!_OS** pelo framework libcosmic
- **Iced** pelo toolkit GUI
- A comunidade **Rust**

---

## 📫 Suporte & Contato

- **Issues**: [GitHub Issues](https://github.com/SEU_USUARIO/cosmic-applet-timeplus/issues)
- **Discussões**: [GitHub Discussions](https://github.com/SEU_USUARIO/cosmic-applet-timeplus/discussions)

---

<p align="center">
Feito com ❤️ para a comunidade COSMIC Desktop
</p>
