/* MIMO i18n — English and Persian. The language is chosen in Settings (key "lang", default "en").
   app.js calls window.MI18N.t(key, vars) for every dynamic string; static HTML carries data-i18n /
   data-i18n-title attributes that setLang() fills in. Switching to Persian mirrors the page (dir="rtl")
   and puts Vazirmatn on (body.fa — the font ships in ui/fonts). */
(() => {
'use strict';
const $$ = (s) => [...document.querySelectorAll(s)];

const DICT = {
  en: {
    // static pill texts
    nothingPlaying: 'Nothing playing', dropToHold: 'Drop to hold', clearAll: 'Clear all',
    clearShelf: 'Clear shelf', clearHistory: 'Clear history', noNotifications: 'No notifications',
    unknownTitle: 'Unknown title', dismiss: 'Dismiss', clearGroup: 'Clear {app}',
    hfTitle: '{name} — drag it out to drop it somewhere, click to show it in its folder',
    // relative time / reminders
    now: 'now', unitM: 'm', unitH: 'h', unitD: 'd', inXm: 'in {n}m', today: 'Today', tomorrow: 'Tomorrow',
    // shelf / files
    dropFilesEmpty: 'Drop files on the pill to hold them',
    fileRowTitle: 'Drag this out to drop the file somewhere — click to show it in its folder',
    takeOffShelf: 'Take off the shelf (the file itself is not touched)',
    holding: 'Holding', holdingN: 'Holding {n} files', dragThemOut: 'drag them out from here',
    cantHold: "Can't hold that", onlyFiles: 'only files and pictures', letGo: 'Let go of it (the file itself is not touched)',
    noted: 'Noted', fileWord: 'file', filesWord: 'files',
    // clipboard
    clipsEmpty: 'Whatever you copy shows up here', clickCopyAgain: 'Click to copy it again',
    unpin: 'Unpin', pinKeep: 'Pin (kept on top, and after a restart)', pinTop: 'Pin on top',
    remove: 'Remove', chars: '{n} characters', copiedPaste: 'Copied — paste with Ctrl+V',
    copied: 'Copied', imageWord: 'image',
    // updates
    updateApp: 'Update', updateDot: 'An update is available',
    versionReady: 'Version {v} is ready', clickInstallLong: 'click to install now (it also installs by itself when you step away)',
    versionAvailable: 'Version {v} is available', clickInstall: 'click to install now',
    wasUpdated: 'MIMO was updated to {v}', whatsNew: "click to see what's new",
    installNow: 'Install now', installing: 'Installing…', installsByItself: 'Installs by itself when you step away',
    takesSeconds: 'Takes a few seconds, then MIMO is back', oneClick: 'One click, then MIMO comes back',
    downloading: 'Downloading…', nothingToInstall: 'Nothing to install', couldNotCheck: 'Could not check',
    checkUpdates: 'Check for updates', checking: 'Checking…', upToDate: 'Up to date', installX: 'Install {v}',
    // network / sensors
    internet: 'Internet', backOnline: 'Back online', noConnection: 'No connection',
    onWord: 'On', offWord: 'Off', proxy: 'Proxy', proxyOn: 'A system proxy is on', offline: 'Offline',
    camera: 'Camera', mic: 'Microphone', inUse: 'in use · {n}', camColon: 'Camera: ', micColon: 'Microphone: ',
    noInternet: 'No internet', vpnColon: 'VPN: ',
    // weather
    weatherWord: 'Weather', noReport: 'No report yet — pick a city in Settings', nowWord: 'Now',
    weatherTipFull: '{name} — click for the days ahead',
    cityPlaceholder: 'City for the weather — Tehran, Paris…', cityNotFound: 'Could not find that city',
    icsPlaceholder: 'A .ics file, or a folder holding them', calendarWord: 'Calendar',
    readingFiles: 'Reading the file(s)', calendarOff: 'Calendar off',
    // events
    soon: 'Soon · {time}', allDay: 'All day', nothingPlanned: 'Nothing planned',
    noCalendarFile: 'No calendar file yet — set one in Settings',
    // lyrics / stats
    noLyrics: 'No lyrics found for this track', playThenLyrics: 'Play something, then open the lyrics',
    cpu: 'CPU', memory: 'Memory', network: 'Network',
    statsOff: 'System stats are off — turn them on below', waitingReading: 'Waiting for the first reading…',
    // timer / battery
    timerApp: 'Timer', timeIsUp: 'Time is up', less: 'Less', more: 'More', startPause: 'Start / Pause', reset: 'Reset',
    charging: 'Charging', onBattery: 'On battery', batteryApp: 'Battery', lowBattery: 'Low battery',
    plugCharger: '{p}% left — plug in your charger',
    // notes
    notePlaceholder: 'Jot it down…  “call Ali in 20m” reminds you',
    reminderX: 'Reminder {when}', notesEmpty: 'Nothing noted yet — type and press Enter',
    noteRowTitle: 'Click to copy — double-click to edit', reminded: 'Reminded', reminderAt: 'Reminder at {t}',
    reminderApp: 'Reminder',
    // vault
    untitled: 'Untitled', typePass: 'Type or paste the password', savedFor: 'Saved. What is it for?  (site or app)',
    emailUser: 'Email or username', couldNotSave: 'Could not save', passOnClipboard: 'the password is still on your clipboard',
    savedAs: 'Saved as {name}', renameAnytime: 'rename it any time under Passwords', savedWord: 'Saved',
    nameNotChanged: 'the name could not be changed', passOnClipShort: 'password is on your clipboard',
    siteOrApp: 'Site or app', updatedWord: 'Updated', couldNotUpdate: 'Could not update',
    couldNotCopy: 'Could not copy', usernameCopied: 'Username copied', passCopied: 'Password copied — clears in 30s',
    search: 'Search', searchTitle: 'Enter copies the password · Shift+Enter the username · Esc clears',
    newBtn: 'New', addBtn: 'Add', newTitle: 'Make a new password', addTitle: 'Save a password you already have',
    closeVault: 'Close the passwords',
    vaultLocked: 'Saved passwords could not be opened on this account',
    vaultEmpty: 'No passwords yet — New makes one', nothingMatches: 'Nothing matches “{q}”',
    vaultRowTitle: 'Click to copy the password (it leaves the clipboard after 30 seconds)',
    copyUser: 'Copy the email / username', copyPass: 'Copy the password', showPass: 'Show the password for a moment',
    renameEntry: 'Rename / change the username', clickAgainDelete: 'Click again to delete', deleteWord: 'Delete',
    keepTip: 'Keep (Enter)', cancelTip: 'Cancel (Esc)',
    // speedo
    signInFirst: 'Sign in inside Speedo first', connecting: 'Connecting…', connected: 'Connected',
    notConnected: 'Not connected', connect: 'Connect', disconnect: 'Disconnect',
    firewall: 'Firewall (block traffic outside the VPN)', connectedHere: 'Connected here',
    connectTo: 'Connect to {name}', couldNotDo: 'Could not do that', couldNotConnect: 'Could not connect',
    disconnected: 'Disconnected',
    // smart copy / ocr / downloads
    clickToOpen: 'click to open', clickToCopy: 'click to copy',
    reading: 'Reading…', textCopied: 'Text copied', textGrab: 'Text grab', nothingFound: 'Nothing found',
    downloadApp: 'Download finished', showInFolder: '· click to show in folder', clickToOpenDot: '· click to open',
    btConnected: 'Connected', btDisconnected: 'Disconnected',
    // settings rows
    sTour: 'What MIMO can do', showMe: 'Show me', autoStart: 'Start with Windows', autoUpdate: 'Update automatically',
    screen: 'Screen', weather: 'Weather', weatherCity: 'Weather city…', set: 'Set', calendarFile: 'Calendar file…',
    eventsBtn: 'Upcoming events', open: 'Open', lyrics: 'Lyrics for the playing track', statsOn: 'System stats',
    pillShape: 'Pill shape', calIsland: 'Island', calNotch: 'Notch',
    restingShows: 'Resting pill shows', eyes: 'Eyes', clock: 'Clock',
    idleWalk: 'Come out and walk when idle', dateWord: 'Date',
    calGreg: 'Greg.', calShamsi: 'Shamsi', calShamsiFa: 'شمسی', calBoth: 'Both',
    autoHide: 'Hide while a window is underneath', hideVideo: 'Stay out of the way for videos',
    theme: 'Theme', musicReact: 'Theme reacts to music', visuals: 'Night sky with music (black theme)',
    notifStays: 'Notification stays for', unitS: 's',
    dndRow: "Hide Windows' own banners", copyPreview: 'Show what was copied', clipHistory: 'Clipboard history',
    chargerBt: 'Charger and Bluetooth', downloadAlerts: 'Finished downloads', shelf: 'File shelf',
    netAlerts: 'Internet and VPN changes', sensorDot: 'Microphone / camera in use',
    smartCopy: 'Smart copy: links, sums, dates', hotkeys: 'Ctrl+Alt+N note · Ctrl+Alt+T text grab',
    language: 'Language · زبان', langEn: 'English', langFa: 'فارسی',
    screenNMain: 'Screen {n} — main',
    plugins: 'Plugins', pluginsFolder: 'Open folder',
    pluginsFolderHint: 'Add cards: one .json file each, in the plugins folder',
    pluginErr: 'Could not read it — check the plugin file', pluginEmpty: 'It answered, but with nothing in it',
    tnBlack: 'Black', tnCover: 'Cover colours', tnPlainBlack: 'Plain black', tnVioletSky: 'Violet night sky',
    tnNebula: 'Nebula', tnCrystal: 'Sapphire', tnEmber: 'Ember', tnReactor: 'Reactor', tnVenom: 'Venom',
    tnCloud: 'Storm cloud', tnWater: 'Water', tnLava: 'Lava', tnIce: 'Ice',
    // dock (a folder of program shortcuts shown as icons)
    dock: 'App dock', dockFolder: 'Open folder', dockHint: 'Programs: put their shortcuts (.lnk, .exe, .url) in the dock folder',
    dockEmpty: 'The dock folder is empty — put program shortcuts in it and they appear here',
    // replying to a notification from the pill
    replyPlaceholder: 'Your reply…', replySent: 'Ready to send', replyPasted: 'the chat is open — paste done, press Enter to send it',
    // tour
    tourHead: 'Hover the pill to open its menu', tryIt: 'Try it',
    tourNowPlaying: 'Now playing', tourNowPlayingD: 'Play something: cover, title and controls appear here',
    tourNotifs: 'Notifications', tourNotifsD: 'One line on the pill. Click it to open that chat',
    tourSmartCopy: 'Smart copy', tourSmartCopyD: 'Copy a link, a sum or a date, then click the line',
    tourShelf: 'File shelf', tourShelfD: 'Drop files on the pill. Drag them out when you need them',
    tourOcr: 'Text grab', tourOcrD: 'Drag a box on the screen, the text is copied',
    tourNote: 'Quick note', tourNoteD: 'One line. “call Ali in 20m” becomes a reminder',
    tourClips: 'Clipboard history', tourClipsD: 'Everything you copied, with images and pins',
    tourVault: 'Passwords', tourVaultD: 'Makes one, asks what it is for, keeps it encrypted',
    tourTimer: 'Timer', tourTimerD: 'Stays in sight while it runs',
    tourThemes: 'Themes and settings', tourThemesD: 'Liquid skins, Venom, and every switch',
    // welcome / tips
    welcomeApp: 'Welcome', hiMimo: "Hi, I'm MIMO", seeWhatICanDo: 'Click to see what I can do',
    tipApp: 'Tip', bannersOnly: 'Banners only here?', dndTip: 'Click, then switch on “Do not disturb”',
    // tooltips of the static pill buttons
    tipDate: 'Click to switch calendar', tipWeather: 'Click for the days ahead',
    tipBack: 'Back to now playing', tipPlayPause: 'Play / Pause', tipShot: 'Screenshot',
    tipOcr: 'Grab text from the screen: drag a box around it (Ctrl+Alt+T)',
    tipClips: 'Clipboard history', tipNote: 'Quick note (Ctrl+Alt+N)', tipVault: 'Passwords', tipReply: 'Reply from here',
    tipSpeedo: 'Speedo VPN', tipTimer: 'Timer',
    tipShelf: 'File shelf — click to see the files, drag this button to take them all',
    tipDock: 'App dock', tipInbox: 'Notifications', tipSettings: 'Settings', tipLyrics: 'Lyrics', tipPrev: 'Previous', tipNext: 'Next',
    tipMore: 'Main menu (stays until you switch back to the track)', tipMute: 'Click to mute',
    tipShowAll: 'Show all the files', tipMenu: 'Menu', tipLess: 'Less', tipMoreSteps: 'More', tipStart: 'Start / Pause',
    tipReset: 'Reset', tipKeep: 'Keep (Enter)', tipCancel: 'Cancel (Esc)', tipAnother: 'Another one',
    tipShorter: 'Shorter', tipLonger: 'Longer', tipSym: 'Symbols on / off',
    tipUse: 'Use it: copies it, then asks where it is for', tipCloseNoSave: 'Close without saving',
    // weather words (WMO codes)
    wClear: 'Clear', wMostlyClear: 'Mostly clear', wPartly: 'Partly cloudy', wCloudy: 'Cloudy',
    wFog: 'Fog', wIcyFog: 'Icy fog', wDrizzle: 'Drizzle', wHeavyDrizzle: 'Heavy drizzle',
    wFreezingDrizzle: 'Freezing drizzle', wLightRain: 'Light rain', wRain: 'Rain', wHeavyRain: 'Heavy rain',
    wFreezingRain: 'Freezing rain', wLightSnow: 'Light snow', wSnow: 'Snow', wHeavySnow: 'Heavy snow',
    wSnowGrains: 'Snow grains', wLightShowers: 'Light showers', wShowers: 'Showers', wHeavyShowers: 'Heavy showers',
    wSnowShowers: 'Snow showers', wStorm: 'Thunderstorm', wStormHail: 'Storm with hail', wStormWord: 'Storm',
  },

  fa: {
    nothingPlaying: 'چیزی پخش نمی‌شود', dropToHold: 'برای نگه‌داشتن رها کنید',
    clearAll: 'پاک کردن همه', clearShelf: 'پاک کردن قفسه', clearHistory: 'پاک کردن تاریخچه',
    noNotifications: 'اعلانی نیست', unknownTitle: 'عنوان ناشناخته', dismiss: 'بستن', clearGroup: 'بستن {app}',
    hfTitle: '{name} — برای رها کردن در جای دیگر بیرون بکشیدش، کلیک برای نمایش در پوشه',
    now: 'الان', unitM: ' د', unitH: ' س', unitD: ' ر', inXm: '{n} دقیقه دیگر',
    today: 'امروز', tomorrow: 'فردا',
    dropFilesEmpty: 'فایل‌ها را روی قرص رها کنید تا نگه داشته شوند',
    fileRowTitle: 'برای رها کردن در جای دیگر بیرون بکشیدش — کلیک برای نمایش در پوشه',
    takeOffShelf: 'برداشتن از قفسه (خود فایل دست‌نخورده می‌ماند)',
    holding: 'نگه داشته شد', holdingN: '{n} فایل نگه داشته شد', dragThemOut: 'از همین‌جا بیرونشان بکشید',
    cantHold: 'این قابل نگه‌داشتن نیست', onlyFiles: 'فقط فایل‌ها و تصاویر',
    letGo: 'رها کردنش (خود فایل دست‌نخورده می‌ماند)',
    noted: 'یادداشت شد', fileWord: 'فایل', filesWord: 'فایل‌ها',
    clipsEmpty: 'هر چیزی که کپی کنید اینجا می‌آید', clickCopyAgain: 'برای کپی دوباره کلیک کنید',
    unpin: 'برداشتن سنجاق', pinKeep: 'سنجاق کردن (بالا می‌ماند، حتی بعد از ری‌استارت)', pinTop: 'سنجاق به بالا',
    remove: 'حذف', chars: '{n} نویسه', copiedPaste: 'کپی شد — با Ctrl+V بچسبانید',
    copied: 'کپی شد', imageWord: 'تصویر',
    updateApp: 'به‌روزرسانی', updateDot: 'به‌روزرسانی موجود است',
    versionReady: 'نسخهٔ {v} آماده است', clickInstallLong: 'برای نصب کلیک کنید (وقتی دست‌کشیدید هم خودش نصب می‌کند)',
    versionAvailable: 'نسخهٔ {v} موجود است', clickInstall: 'برای نصب کلیک کنید',
    wasUpdated: 'MIMO به {v} به‌روزرسانی شد', whatsNew: 'برای دیدن تغییرات کلیک کنید',
    installNow: 'الان نصب کن', installing: 'در حال نصب…', installsByItself: 'وقتی دست‌کشیدید خودش نصب می‌کند',
    takesSeconds: 'چند ثانیه طول می‌کشد و MIMO برمی‌گردد', oneClick: 'یک کلیک، و MIMO برمی‌گردد',
    downloading: 'در حال دانلود…', nothingToInstall: 'چیزی برای نصب نیست', couldNotCheck: 'بررسی ممکن نشد',
    checkUpdates: 'بررسی به‌روزرسانی', checking: 'در حال بررسی…', upToDate: 'به‌روز است', installX: 'نصب {v}',
    internet: 'اینترنت', backOnline: 'دوباره وصل شد', noConnection: 'بدون اتصال',
    onWord: 'روشن', offWord: 'خاموش', proxy: 'پروکسی', proxyOn: 'یک پروکسی سیستمی روشن است', offline: 'بی‌اینترنت',
    camera: 'دوربین', mic: 'میکروفون', inUse: 'در حال استفاده · {n}', camColon: 'دوربین: ', micColon: 'میکروفون: ',
    noInternet: 'بدون اینترنت', vpnColon: 'VPN: ',
    weatherWord: 'هوا', noReport: 'هنوز گزارشی نیست — از تنظیمات شهر را انتخاب کنید', nowWord: 'الان',
    weatherTipFull: '{name} — برای روزهای پیش‌رو کلیک کنید',
    cityPlaceholder: 'شهر برای هوا — تهران، پاریس…', cityNotFound: 'آن شهر پیدا نشد',
    icsPlaceholder: 'یک فایل .ics، یا پوشه‌ای از آن‌ها', calendarWord: 'تقویم',
    readingFiles: 'در حال خواندن فایل(ها)', calendarOff: 'تقویم خاموش است',
    soon: 'به‌زودی · {time}', allDay: 'تمام روز', nothingPlanned: 'برنامه‌ای نیست',
    noCalendarFile: 'فایل تقویمی نیست — از تنظیمات انتخابش کنید',
    noLyrics: 'برای این آهنگ متنی پیدا نشد', playThenLyrics: 'چیزی پخش کنید، بعد متن را باز کنید',
    cpu: 'پردازنده', memory: 'حافظه', network: 'شبکه',
    statsOff: 'آمار سیستم خاموش است — از پایین روشنش کنید', waitingReading: 'در انتظار اولین اندازه‌گیری…',
    timerApp: 'تایمر', timeIsUp: 'زمان تمام شد', less: 'کمتر', more: 'بیشتر', startPause: 'شروع / توقف', reset: 'از نو',
    charging: 'در حال شارژ', onBattery: 'روی باتری', batteryApp: 'باتری', lowBattery: 'باتری کم است',
    plugCharger: '{p}% مانده — شارژر را وصل کنید',
    notePlaceholder: 'بنویسید… «علی را تا ۲۰ دقیقه دیگر صدا بزن» یادآور می‌شود',
    reminderX: 'یادآور {when}', notesEmpty: 'هنوز چیزی نوشته نشده — بنویسید و Enter بزنید',
    noteRowTitle: 'کلیک برای کپی — دابل‌کلیک برای ویرایش', reminded: 'یادآوری شد', reminderAt: 'یادآور در {t}',
    reminderApp: 'یادآور',
    untitled: 'بی‌نام', typePass: 'رمز را بنویسید یا بچسبانید', savedFor: 'ذخیره شد. برای چیست؟ (سایت یا برنامه)',
    emailUser: 'ایمیل یا نام کاربری', couldNotSave: 'ذخیره نشد', passOnClipboard: 'رمز هنوز در کلیپ‌بورد شماست',
    savedAs: 'با نام {name} ذخیره شد', renameAnytime: 'هر وقت خواستید در بخش رمزها عوضش کنید', savedWord: 'ذخیره شد',
    nameNotChanged: 'نام عوض نشد', passOnClipShort: 'رمز در کلیپ‌بورد است',
    siteOrApp: 'سایت یا برنامه', updatedWord: 'به‌روز شد', couldNotUpdate: 'به‌روزرسانی نشد',
    couldNotCopy: 'کپی نشد', usernameCopied: 'نام کاربری کپی شد', passCopied: 'رمز کپی شد — بعد از ۳۰ ثانیه پاک می‌شود',
    search: 'جست‌وجو', searchTitle: 'Enter رمز را کپی می‌کند · Shift+Enter نام کاربری · Esc پاک می‌کند',
    newBtn: 'جدید', addBtn: 'افزودن', newTitle: 'ساخت رمز تازه', addTitle: 'ذخیرهٔ رمزی که از قبل دارید',
    closeVault: 'بستن رمزها',
    vaultLocked: 'رمزهای ذخیره‌شده در این حساب باز نشدند',
    vaultEmpty: 'هنوز رمزی نیست — «جدید» یکی می‌سازد', nothingMatches: 'چیزی با «{q}» پیدا نشد',
    vaultRowTitle: 'کلیک برای کپی رمز (بعد از ۳۰ ثانیه از کلیپ‌بورد پاک می‌شود)',
    copyUser: 'کپی ایمیل / نام کاربری', copyPass: 'کپی رمز', showPass: 'نمایش رمز برای لحظه‌ای',
    renameEntry: 'تغییر نام / نام کاربری', clickAgainDelete: 'برای حذف دوباره کلیک کنید', deleteWord: 'حذف',
    keepTip: 'نگه‌داشتن (Enter)', cancelTip: 'لغو (Esc)',
    signInFirst: 'اول داخل Speedo وارد شوید', connecting: 'در حال اتصال…', connected: 'وصل شد',
    notConnected: 'وصل نیست', connect: 'اتصال', disconnect: 'قطع',
    firewall: 'فایروال (مسدود کردن ترافیک بیرون از VPN)', connectedHere: 'اینجا وصل هستید',
    connectTo: 'اتصال به {name}', couldNotDo: 'انجام نشد', couldNotConnect: 'اتصال ممکن نشد',
    disconnected: 'قطع شد',
    clickToOpen: 'برای باز کردن کلیک کنید', clickToCopy: 'برای کپی کلیک کنید',
    reading: 'در حال خواندن…', textCopied: 'متن کپی شد', textGrab: 'برداشتن متن', nothingFound: 'چیزی پیدا نشد',
    downloadApp: 'دانلود تمام شد', showInFolder: '· کلیک برای نمایش در پوشه', clickToOpenDot: '· کلیک برای باز کردن',
    btConnected: 'وصل شد', btDisconnected: 'قطع شد',
    sTour: 'کارهایی که MIMO می‌کند', showMe: 'ببینم', autoStart: 'اجرای خودکار با ویندوز', autoUpdate: 'به‌روزرسانی خودکار',
    screen: 'نمایشگر', weather: 'هوا', weatherCity: 'شهر هوا…', set: 'انتخاب', calendarFile: 'فایل تقویم…',
    eventsBtn: 'رویدادهای پیش‌رو', open: 'باز کردن', lyrics: 'متن آهنگ در حال پخش', statsOn: 'آمار سیستم',
    pillShape: 'شکل قرص', calIsland: 'جزیره', calNotch: 'بریدگی',
    restingShows: 'قرص در حالت سکون', eyes: 'چشم‌ها', clock: 'ساعت',
    idleWalk: 'وقتی کار نمی‌کنید بیرون بیاید و قدم بزند', dateWord: 'تاریخ',
    calGreg: 'میلادی', calShamsi: 'شمسی لاتین', calShamsiFa: 'شمسی فارسی', calBoth: 'هر دو',
    autoHide: 'وقتی پنجره‌ای زیرش است پنهان شود', hideVideo: 'برای ویدیوها کنار برود',
    theme: 'پوسته', musicReact: 'پوسته به موسیقی واکنش نشان دهد', visuals: 'آسمان شب با موسیقی (پوستهٔ سیاه)',
    notifStays: 'طول نمایش اعلان', unitS: ' ث',
    dndRow: 'پنهان کردن بنرهای خود ویندوز', copyPreview: 'نمایش چیزی که کپی شد', clipHistory: 'تاریخچهٔ کلیپ‌بورد',
    chargerBt: 'شارژر و بلوتوث', downloadAlerts: 'دانلودهای تمام‌شده', shelf: 'قفسهٔ فایل',
    netAlerts: 'تغییرات اینترنت و VPN', sensorDot: 'میکروفون / دوربین در حال استفاده',
    smartCopy: 'کپی هوشمند: لینک‌ها، جمع‌ها، تاریخ‌ها', hotkeys: 'Ctrl+Alt+N یادداشت · Ctrl+Alt+T برداشتن متن',
    language: 'Language · زبان', langEn: 'English', langFa: 'فارسی',
    screenNMain: 'نمایشگر {n} — اصلی',
    plugins: 'افزونه‌ها', pluginsFolder: 'باز کردن پوشه',
    pluginsFolderHint: 'کارت اضافه کنید: هر افزونه یک فایل .json در پوشهٔ افزونه‌ها',
    pluginErr: 'خواندنش ممکن نشد — فایل افزونه را بررسی کنید', pluginEmpty: 'جواب داد، ولی خالی بود',
    tnBlack: 'سیاه', tnCover: 'رنگ‌های جلد', tnPlainBlack: 'سیاه ساده', tnVioletSky: 'آسمان شب بنفش',
    tnNebula: 'سحابی', tnCrystal: 'یاقوت', tnEmber: 'اخگر', tnReactor: 'رآکتور', tnVenom: 'ونوم',
    tnCloud: 'ابر طوفانی', tnWater: 'آب', tnLava: 'گدازه', tnIce: 'یخ',
    dock: 'نوار برنامه‌ها', dockFolder: 'باز کردن پوشه', dockHint: 'برنامه‌ها: میان‌برشان (.lnk، .exe، .url) را در پوشهٔ نوار بگذارید',
    dockEmpty: 'پوشهٔ نوار خالی است — میان‌بر برنامه‌ها را درش بگذارید تا اینجا بیایند',
    replyPlaceholder: 'پاسخ شما…', replySent: 'آمادهٔ ارسال', replyPasted: 'گفت‌وگو باز شد و چسبانده شد — با Enter بفرست',
    tourHead: 'برای باز کردن منو، نشانگر را روی قرص ببرید', tryIt: 'امتحان کنید',
    tourNowPlaying: 'در حال پخش', tourNowPlayingD: 'چیزی پخش کنید: جلد، عنوان و دکمه‌ها اینجا می‌آیند',
    tourNotifs: 'اعلان‌ها', tourNotifsD: 'یک خط روی قرص. کلیکش کنید تا همان گفت‌وگو باز شود',
    tourSmartCopy: 'کپی هوشمند', tourSmartCopyD: 'لینک، جمع یا تاریخ را کپی کنید، بعد روی خط کلیک کنید',
    tourShelf: 'قفسهٔ فایل', tourShelfD: 'فایل‌ها را روی قرص رها کنید. هر وقت لازم شد بیرون بکشید',
    tourOcr: 'برداشتن متن', tourOcrD: 'کادری روی صفحه بکشید، متن کپی می‌شود',
    tourNote: 'یادداشت سریع', tourNoteD: 'یک خط. «علی را تا ۲۰ دقیقه دیگر صدا بزن» یادآور می‌شود',
    tourClips: 'تاریخچهٔ کلیپ‌بورد', tourClipsD: 'هر چه کپی کرده‌اید، با تصاویر و سنجاق‌ها',
    tourVault: 'رمزها', tourVaultD: 'رمز می‌سازد، می‌پرسد برای چیست، و رمزنگاری‌شده نگه می‌دارد',
    tourTimer: 'تایمر', tourTimerD: 'تا وقتی کار می‌کند در دید می‌ماند',
    tourThemes: 'پوسته‌ها و تنظیمات', tourThemesD: 'پوسته‌های مایع، ونوم، و همهٔ کلیدها',
    welcomeApp: 'خوش آمدید', hiMimo: 'سلام، من MIMO هستم', seeWhatICanDo: 'برای دیدن کارهایم کلیک کنید',
    tipApp: 'نکته', bannersOnly: 'بنرها فقط اینجا می‌آیند؟', dndTip: 'کلیک کنید، بعد «مزاحم نشو» را روشن کنید',
    tipDate: 'کلیک برای تغییر تقویم', tipWeather: 'کلیک برای روزهای پیش‌رو',
    tipBack: 'بازگشت به پخش', tipPlayPause: 'پخش / توقف', tipShot: 'عکس صفحه',
    tipOcr: 'برداشتن متن از صفحه: دورش کادر بکشید (Ctrl+Alt+T)',
    tipClips: 'تاریخچهٔ کلیپ‌بورد', tipNote: 'یادداشت سریع (Ctrl+Alt+N)', tipVault: 'رمزها', tipReply: 'پاسخ از همین‌جا',
    tipSpeedo: 'Speedo VPN', tipTimer: 'تایمر',
    tipShelf: 'قفسهٔ فایل — کلیک برای دیدن فایل‌ها، بیرون کشیدن این دکمه برای بردن همه',
    tipDock: 'نوار برنامه‌ها', tipInbox: 'اعلان‌ها', tipSettings: 'تنظیمات', tipLyrics: 'متن آهنگ', tipPrev: 'قبلی', tipNext: 'بعدی',
    tipMore: 'منوی اصلی (تا وقتی به آهنگ برنگردید می‌ماند)', tipMute: 'کلیک برای بی‌صدا',
    tipShowAll: 'نمایش همهٔ فایل‌ها', tipMenu: 'منو', tipLess: 'کمتر', tipMoreSteps: 'بیشتر', tipStart: 'شروع / توقف',
    tipReset: 'از نو', tipKeep: 'نگه‌داشتن (Enter)', tipCancel: 'لغو (Esc)', tipAnother: 'یکی دیگر',
    tipShorter: 'کوتاه‌تر', tipLonger: 'بلندتر', tipSym: 'نمادها روشن / خاموش',
    tipUse: 'استفاده: کپی می‌کند، بعد می‌پرسد برای چیست', tipCloseNoSave: 'بستن بدون ذخیره',
    wClear: 'صاف', wMostlyClear: 'عمدتاً صاف', wPartly: 'نیمه‌ابری', wCloudy: 'ابری',
    wFog: 'مه', wIcyFog: 'مه یخ‌زده', wDrizzle: 'نم‌نم باران', wHeavyDrizzle: 'نم‌نم شدید',
    wFreezingDrizzle: 'نم‌نم یخ‌زده', wLightRain: 'باران سبک', wRain: 'باران', wHeavyRain: 'باران شدید',
    wFreezingRain: 'باران یخ‌زده', wLightSnow: 'برف سبک', wSnow: 'برف', wHeavySnow: 'برف سنگین',
    wSnowGrains: 'دانه برف', wLightShowers: 'رگبار سبک', wShowers: 'رگبار', wHeavyShowers: 'رگبار شدید',
    wSnowShowers: 'رگبار برف', wStorm: 'رعدوبرق', wStormHail: 'طوفان با تگرگ', wStormWord: 'طوفان',
  },
};

let lang = 'en';

// t('key') or t('key', { v: '1.2.3' }) — {placeholders} are replaced; unknown keys come back as themselves
const t = (key, vars) => {
  let s = (DICT[lang] && DICT[lang][key]) ?? DICT.en[key] ?? key;
  if (vars) for (const k in vars) s = s.split('{' + k + '}').join(String(vars[k]));
  return s;
};

// fills the data-i18n (textContent) and data-i18n-title (title / the pill's own tooltip) attributes of the static HTML
const paintStatic = () => {
  $$('[data-i18n]').forEach((el) => { el.textContent = t(el.dataset.i18n); });
  $$('[data-i18n-title]').forEach((el) => {
    if (el.dataset.tip !== undefined) el.dataset.tip = t(el.dataset.i18nTitle);
    else el.title = t(el.dataset.i18nTitle);
  });
};

// extra repainting for strings that are not in the static HTML (badges, dynamic buttons)
const repaints = [];
const onRepaint = (fn) => repaints.push(fn);

const setLang = (l) => {
  lang = l === 'fa' ? 'fa' : 'en';
  const de = document.documentElement;
  de.lang = lang;
  de.dir = lang === 'fa' ? 'rtl' : 'ltr';
  document.body.classList.toggle('fa', lang === 'fa');
  paintStatic();
  repaints.forEach((fn) => { try { fn(); } catch { /* one broken painter must not stop the rest */ } });
};

window.MI18N = { t, setLang, lang: () => lang, is: (l) => lang === l, onRepaint };
})();
