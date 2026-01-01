#!/bin/bash
# Script to create cosmic_applet_timeplus.ftl for all languages
# Based on the English template

# English template (reference)
TEMPLATE_EN="/home/cysneiros/Template Cosmic Applet/cosmic-applet-template/cosmic-applet-timeplus/i18n/en/cosmic_applet_timeplus.ftl"

# Base directory
I18N_DIR="/home/cysneiros/Template Cosmic Applet/cosmic-applet-template/cosmic-applet-timeplus/i18n"

# Translations map (language_code:translation)
# Format: "lang:calendar|weather|timer|datetime-settings|weather-subtitle|timer-subtitle|weather-msg|timer-msg|features|..."

declare -A TRANSLATIONS

# Major European languages
TRANSLATIONS[de]="Kalender|Wetter|Timer|Datum-, Zeit- und Kalendereinstellungen...|Wetter- und Klima-Widget|Timer-Widget|Wetter- und Klimainformationsintegration! (Demnächst!)|Pomodoro und benutzerdefinierte Timer! (Demnächst!)|Geplante Funktionen:|Aktuelle Wetterbedingungen|Temperatur und gefühlte Temperatur|Mehrtägige Vorhersage|Anpassbarer Countdown-Timer|Schnellvoreinstellungen (Pomodoro, kurze Pause, lange Pause)|Desktop-Benachrichtigungen bei Abschluss|Sitzungspersistenz über Neustarts hinweg"

TRANSLATIONS[es-ES]="Calendario|Clima|Temporizador|Configuración de fecha, hora y calendario...|Widget de Clima|Widget de Temporizador|¡Integración de información meteorológica y climática! (¡Próximamente!)|¡Pomodoro y temporizadores personalizados! (¡Próximamente!)|Características planificadas:|Condiciones meteorológicas actuales|Temperatura y sensación térmica|Pronóstico de varios días|Temporizador de cuenta regresiva personalizable|Preajustes rápidos (Pomodoro, pausa corta, pausa larga)|Notificaciones de escritorio al completarse|Persistencia de sesión entre reinicios"

TRANSLATIONS[fr]="Calendrier|Météo|Minuteur|Paramètres de date, heure et calendrier...|Widget Météo et Climat|Widget Minuteur|Intégration des informations météorologiques et climatiques ! (Bientôt !)|Pomodoro et minuteurs personnalisés ! (Bientôt !)|Fonctionnalités prévues :|Conditions météorologiques actuelles|Température et ressenti|Prévisions sur plusieurs jours|Minuteur à rebours personnalisable|Préréglages rapides (Pomodoro, pause courte, pause longue)|Notifications de bureau à la fin|Persistance de session entre les redémarrages"

TRANSLATIONS[it]="Calendario|Meteo|Timer|Impostazioni di data, ora e calendario...|Widget Meteo e Clima|Widget Timer|Integrazione informazioni meteo e clima! (Prossimamente!)|Pomodoro e timer personalizzati! (Prossimamente!)|Funzionalità pianificate:|Condizioni meteo attuali|Temperatura e temperatura percepita|Previsioni multi-giorno|Timer conto alla rovescia personalizzabile|Preset rapidi (Pomodoro, pausa breve, pausa lunga)|Notifiche desktop al completamento|Persistenza sessione tra riavvii"

TRANSLATIONS[ru]="Календарь|Погода|Таймер|Настройки даты, времени и календаря...|Виджет погоды и климата|Виджет таймера|Интеграция информации о погоде и климате! (Скоро!)|Pomodoro и пользовательские таймеры! (Скоро!)|Планируемые функции:|Текущие погодные условия|Температура и ощущаемая температура|Многодневный прогноз|Настраиваемый таймер обратного отсчета|Быстрые пресеты (Pomodoro, короткий перерыв, длинный перерыв)|Уведомления на рабочем столе при завершении|Сохранение сеанса между перезагрузками"

TRANSLATIONS[ja]="カレンダー|天気|タイマー|日付、時刻、カレンダーの設定...|天気と気候ウィジェット|タイマーウィジェット|天気と気候情報の統合！（近日公開！）|ポモドーロとカスタムタイマー！（近日公開！）|予定されている機能：|現在の気象条件|気温と体感温度|複数日の予報|カスタマイズ可能なカウントダウンタイマー|クイックプリセット（ポモドーロ、短い休憩、長い休憩）|完了時のデスクトップ通知|再起動後のセッション永続化"

TRANSLATIONS[zh-CN]="日历|天气|计时器|日期、时间和日历设置...|天气和气候小部件|计时器小部件|天气和气候信息集成！（即将推出！）|番茄钟和自定义计时器！（即将推出！）|计划功能：|当前天气状况|温度和体感温度|多日预报|可自定义倒计时器|快速预设（番茄钟、短休息、长休息）|完成时桌面通知|重启后会话持久化"

TRANSLATIONS[ko]="달력|날씨|타이머|날짜, 시간 및 달력 설정...|날씨 및 기후 위젯|타이머 위젯|날씨 및 기후 정보 통합! (곧 출시!)|뽀모도로 및 사용자 정의 타이머! (곧 출시!)|계획된 기능:|현재 날씨 상태|온도 및 체감 온도|여러 날 예보|사용자 정의 가능한 카운트다운 타이머|빠른 사전 설정 (뽀모도로, 짧은 휴식, 긴 휴식)|완료 시 데스크톱 알림|재부팅 후 세션 지속성"

# For remaining languages, use English as fallback
echo "Creating cosmic_applet_timeplus.ftl files for all languages..."

for lang_dir in "$I18N_DIR"/*/ ; do
    lang=$(basename "$lang_dir")
    target_file="${lang_dir}cosmic_applet_timeplus.ftl"
    
    # Skip if already exists (en and pt-BR)
    if [ -f "$target_file" ]; then
        echo "✓ $lang (already exists)"
        continue
    fi
    
    # Check if we have translation
    if [ -n "${TRANSLATIONS[$lang]}" ]; then
        IFS='|' read -ra TRANS <<< "${TRANSLATIONS[$lang]}"
        cat > "$target_file" << EOF
calendar = ${TRANS[0]}
weather = ${TRANS[1]}
timer = ${TRANS[2]}
datetime-settings = ${TRANS[3]}

# Weather placeholder
weather = ${TRANS[1]}
weather-subtitle = ${TRANS[4]}
weather-placeholder-message = ${TRANS[6]}
weather-placeholder-features = ${TRANS[8]}
weather-feature-current = ${TRANS[9]}
weather-feature-temperature = ${TRANS[10]}
weather-feature-forecast = ${TRANS[11]}

# Timer placeholder
timer = ${TRANS[2]}
timer-subtitle = ${TRANS[5]}
timer-placeholder-message = ${TRANS[7]}
timer-placeholder-features = ${TRANS[8]}
timer-feature-countdown = ${TRANS[12]}
timer-feature-presets = ${TRANS[13]}
timer-feature-notifications = ${TRANS[14]}
timer-feature-persistent = ${TRANS[15]}

EOF
        echo "✓ $lang (translated)"
    else
        # Use English as fallback
        cp "$TEMPLATE_EN" "$target_file"
        echo "✓ $lang (English fallback)"
    fi
done

echo ""
echo "✅ All language files created!"
echo "📝 Translated: de, es-ES, fr, it, ru, ja, zh-CN, ko"
echo "📝 English fallback: all other languages"
