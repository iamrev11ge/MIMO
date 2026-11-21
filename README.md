<div align="center">

# MIMO

**A tiny dynamic island for Windows.** One small pill at the top of your screen that shows what is playing,
catches your notifications, remembers what you copied, holds your files — and when you step away, it comes
out for a little walk.

[![release](https://img.shields.io/github/v/release/iamrev11ge/MIMO?label=release&color=555)](https://github.com/iamrev11ge/MIMO/releases/latest)
[![downloads](https://img.shields.io/github/downloads/iamrev11ge/MIMO/total?label=downloads&color=555)](https://github.com/iamrev11ge/MIMO/releases)
[![build](https://img.shields.io/github/actions/workflow/status/iamrev11ge/MIMO/build.yml?branch=main&label=build)](https://github.com/iamrev11ge/MIMO/actions/workflows/build.yml)
![size](https://img.shields.io/badge/size-~3%20MB-555)
[![license](https://img.shields.io/badge/license-MIT-555)](LICENSE)

[![Windows installer](https://img.shields.io/badge/WINDOWS-INSTALLER-0a7cd6?style=for-the-badge)](https://github.com/iamrev11ge/MIMO/releases/latest/download/MIMO-Setup.exe)
[![Windows portable exe](https://img.shields.io/badge/WINDOWS-PORTABLE%20.EXE-1f2328?style=for-the-badge)](https://github.com/iamrev11ge/MIMO/releases/latest/download/MIMO-Portable.exe)
[![Windows zip](https://img.shields.io/badge/WINDOWS-.ZIP-1f2328?style=for-the-badge)](https://github.com/iamrev11ge/MIMO/releases/latest/download/MIMO-Windows.zip)

[فارسی](#فارسی)

</div>

<img width="1585" height="992" alt="hero" src="https://github.com/user-attachments/assets/cc1f8c13-bede-4566-836e-b7b5df7c74ad" />

## What it does

- **Now playing.** Title, artist, cover, controls and a scrubber for whatever is playing on the system. A track that starts shows itself for a few seconds — long enough to skip or stop it — and stays while the cursor rests on the pill. Films and fullscreen games are left alone.
- **A little walk.** After thirty seconds of quiet the character comes out: a small dark body, glowing eyes, a gentle waddle across the screen. Any touch of the mouse or keyboard and it walks home. It can be switched off in the settings.
- **Themes that move.** Liquid skins that drift at rest and flood with colour on the beat (or stay calm, if you switch that off). In **Venom** the pill itself turns to black goo.
- **Notifications in one line.** Click to open that chat. While you are using the pill they wait beside it instead of taking it over.
- **One hover away.** Screenshot, text grab, clipboard history, quick note, passwords, timer, notifications, settings.
- **Smart copy.** A copied link can be opened, a sum gets its answer, a date its twin in the other calendar (Gregorian ⇄ Shamsi), a colour its swatch.
- **File shelf.** Drop files on the pill; up to five sit there side by side until you drag them out again.
- **Timer.** Stays in sight while it runs, even over a fullscreen app.
- **Settings inside the island.** No extra windows.

And also:

- **Quick notes.** `Ctrl+Alt+N` from anywhere, type one line, Enter. A time in the line (`call Ali in 20m`, `at 18:30 gym`, «۱۰ دقیقه دیگه چای») turns it into a reminder.
- **Passwords.** Generate one, confirm, and it asks which site and which e-mail it is for. Stored encrypted with your Windows account (DPAPI); copies clear themselves from the clipboard after 30 seconds.
- **Text grab.** `Ctrl+Alt+T`, drag a box, and the text in it is on your clipboard. English is read by Windows' own OCR; Persian by a bundled Tesseract. Nothing leaves your machine.
- **Clipboard history** with images and pins.
- **Small signs.** A dot while the microphone or camera is in use, a line when the internet or a VPN goes up or down, Bluetooth devices and the charger.
- **Updates itself.** New releases are downloaded from this repository, checked against the project's signing key, and installed with one click — or by themselves when you step away, unless "Update automatically" is switched off.

## Install

Download **[MIMO-Setup.exe](https://github.com/iamrev11ge/MIMO/releases/download/v1.0.1/MIMO.exe)** and run it.
It installs for the current user only (no administrator prompt) and starts with Windows; both can be changed in the settings.

Windows 10 or 11 with the WebView2 runtime (already part of Windows 11 and of current Windows 10).

The installer is not code-signed, so Windows SmartScreen may ask once whether to run it.

## Privacy

MIMO makes no network requests except checking this repository for a new release (which can be switched off).
Notifications, clipboard contents, notes and passwords stay on your computer.

## Build it yourself

```
cd src-tauri
cargo build --release
```

The result is a single `mimo.exe` (about 5 MB). The installer is `installer/island.nsi` (NSIS).
Every push is built by [GitHub Actions](.github/workflows/build.yml); a tag `vX.Y.Z` publishes a release
titled by its major and minor — `v1.0.0` becomes *MIMO 1.0*, `v1.1.0` becomes *MIMO 1.1*.

### Your own signature

Two kinds, both optional and both yours:

- **Update signatures (Ed25519).** The updater refuses any installer that is not signed by the project's
  key. Make your own pair with `node installer/make-keys.mjs`, put the printed private key in the repo
  secret `MIMO_SIGN_KEY` and paste the printed array into `KEY` in `src-tauri/src/update.rs`.
- **Windows signature (Authenticode).** This is what makes Explorer say "verified publisher".
  `powershell -File installer\make-cert.ps1 -Password '...'` makes you a code-signing certificate;
  `powershell -File installer\sign-code.ps1` signs the binaries (set `MIMO_PFX` and `MIMO_PFX_PASS` first).
  In CI set the secrets `MIMO_PFX_B64` (the .pfx, base64) and `MIMO_PFX_PASS` and releases sign themselves.
  Azure Trusted Signing (~$10/month) and SignPath (free for OSS) are wired too — the full comparison
  and the exact setup steps live in [docs/signing.md](docs/signing.md).

## Platforms

Windows today. A Linux version is being worked on; the backend talks to Windows directly for almost everything
(notifications, media session, clipboard, the click-through window), so it is a port, not a rebuild.

## License

[MIT](LICENSE). Bundles tesseract.js and Tesseract's Persian data (Apache 2.0).

---

<div dir="rtl">

## فارسی

**MIMO یک جزیرهٔ کوچک و زنده برای ویندوز است.** یک قرص کوچک بالای صفحه که آهنگ در حال پخش را نشان می‌دهد،
نوتیفیکیشن‌ها را می‌گیرد، چیزهایی که کپی کرده‌ای یادش می‌ماند، فایل‌هایت را نگه می‌دارد — و وقتی رهایش کنی،
می‌آید بیرون و کمی قدم می‌زند.


### امکانات

- **پخش موزیک:** نام آهنگ، خواننده، کاور، دکمه‌های کنترل و نوار جلو و عقب بردن. با شروع هر آهنگ چند ثانیه خودش را نشان می‌دهد تا بتوانی عوضش کنی یا استپش بزنی. با فیلم و بازی تمام‌صفحه کاری ندارد.
- **قدم‌زدن:** بعد از سی ثانیه بی‌کاری، کاراکتر می‌آید بیرون — بدنهٔ کوچک تیره، چشم‌های درخشان و قدم‌های آرام روی صفحه. با اولین حرکت موس یا کیبورد به خانه برمی‌گردد. از تنظیمات می‌شود خاموشش کرد.
- **تم‌های متحرک:** پوست‌های مایع که آرام حرکت می‌کنند و با ضرب آهنگ پر از رنگ می‌شوند (می‌شود واکنش به موزیک را خاموش کرد).
- **ونوم:** خود جزیره به لجن سیاه زنده تبدیل می‌شود.
- **نوتیفیکیشن در یک خط:** با کلیک همان چت باز می‌شود. وقتی مشغول کار با جزیره‌ای، کنار آن منتظر می‌ماند.
- **کپی هوشمند:** لینکِ کپی‌شده باز می‌شود، حساب جواب می‌گیرد، تاریخ میلادی و شمسی به هم تبدیل می‌شوند.
- **نگه‌داشتن فایل:** فایل را روی جزیره رها کن؛ کنار هم می‌مانند تا دوباره بیرونشان بکشی.
- **یادداشت سریع:** `Ctrl+Alt+N` از هر جا. اگر در متن زمان باشد («۱۰ دقیقه دیگه چای») یادآور می‌شود.
- **رمزها:** رمز می‌سازد، سایت و ایمیل را می‌پرسد و ذخیره می‌کند (رمزنگاری‌شده با حساب ویندوز).
- **خواندن متن از صفحه:** `Ctrl+Alt+T`، دور متن کادر بکش؛ انگلیسی و فارسی. همه‌چیز روی سیستم خودت انجام می‌شود.
- **تایمر، تاریخچهٔ کپی با عکس، نشانگر میکروفون و دوربین، وضعیت اینترنت و VPN.**
- **آپدیت خودکار:** نسخهٔ جدید از همین مخزن دانلود، با کلید امضای پروژه بررسی و با یک کلیک نصب می‌شود — یا خودش، وقتی پشت سیستم نباشی (مگر اینکه «Update automatically» را خاموش کرده باشی).

### نصب

فایل **[MIMO-Setup.exe](https://github.com/iamrev11ge/MIMO/releases/download/v1.0.1/MIMO.exe)** را دانلود و اجرا کن.
فقط برای کاربر فعلی نصب می‌شود و دسترسی ادمین نمی‌خواهد.

### حریم خصوصی

MIMO هیچ درخواست اینترنتی‌ای نمی‌فرستد، به‌جز بررسی همین مخزن برای نسخهٔ جدید (که می‌شود خاموشش کرد).
نوتیفیکیشن‌ها، کلیپ‌بورد، یادداشت‌ها و رمزها روی سیستم خودت می‌مانند.

</div>
