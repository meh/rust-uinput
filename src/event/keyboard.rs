use libc::c_int;
use ffi::*;
use {Event};
use super::{Kind, Code, Press, Release};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Keyboard {
	All,
	Key(Key),
	KeyPad(KeyPad),
	Misc(Misc),
	InputAssist(InputAssist),
	Function(Function),
	Braille(Braille),
	Numeric(Numeric),
	TouchPad(TouchPad),
	Camera(Camera),
	Attendant(Attendant),
}

impl Into<Event> for Keyboard {
	fn into(self) -> Event {
		Event::Keyboard(self)
	}
}

impl Press for Keyboard { }
impl Release for Keyboard { }

impl Kind for Keyboard {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Keyboard {
	fn code(&self) -> c_int {
		match self {
			&Keyboard::All => unreachable!(),

			&Keyboard::Key(ref v)         => v.code(),
			&Keyboard::KeyPad(ref v)      => v.code(),
			&Keyboard::Misc(ref v)        => v.code(),
			&Keyboard::InputAssist(ref v) => v.code(),
			&Keyboard::Function(ref v)    => v.code(),
			&Keyboard::Braille(ref v)     => v.code(),
			&Keyboard::Numeric(ref v)     => v.code(),
			&Keyboard::TouchPad(ref v)    => v.code(),
			&Keyboard::Camera(ref v)      => v.code(),
			&Keyboard::Attendant(ref v)   => v.code(),
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(KeyVariants))]
	pub enum Key {
		Reserved,
		Esc,
		_1,
		_2,
		_3,
		_4,
		_5,
		_6,
		_7,
		_8,
		_9,
		_0,
		Minus,
		Equal,
		BackSpace,
		Tab,
		Q,
		W,
		E,
		R,
		T,
		Y,
		U,
		I,
		O,
		P,
		LeftBrace,
		RightBrace,
		Enter,
		LeftControl,
		A,
		S,
		D,
		F,
		G,
		H,
		J,
		K,
		L,
		SemiColon,
		Apostrophe,
		Grave,
		LeftShift,
		BackSlash,
		Z,
		X,
		C,
		V,
		B,
		N,
		M,
		Comma,
		Dot,
		Slash,
		RightShift,
		LeftAlt,
		Space,
		CapsLock,
		F1,
		F2,
		F3,
		F4,
		F5,
		F6,
		F7,
		F8,
		F9,
		F10,
		NumLock,
		ScrollLock,
		F11,
		F12,
		RightControl,
		SysRq,
		RightAlt,
		LineFeed,
		Home,
		Up,
		PageUp,
		Left,
		Right,
		End,
		Down,
		PageDown,
		Insert,
		Delete,
		LeftMeta,
		RightMeta,
		ScrollUp,
		ScrollDown,
		F13,
		F14,
		F15,
		F16,
		F17,
		F18,
		F19,
		F20,
		F21,
		F22,
		F23,
		F24,
	}
}

impl Into<Event> for Key {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Key(self))
	}
}

impl Press for Key { }
impl Release for Key { }

impl Kind for Key {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Key {
	fn code(&self) -> c_int {
		match self {
			&Key::Reserved         => KEY_RESERVED,
			&Key::Esc              => KEY_ESC,
			&Key::_1               => KEY_1,
			&Key::_2               => KEY_2,
			&Key::_3               => KEY_3,
			&Key::_4               => KEY_4,
			&Key::_5               => KEY_5,
			&Key::_6               => KEY_6,
			&Key::_7               => KEY_7,
			&Key::_8               => KEY_8,
			&Key::_9               => KEY_9,
			&Key::_0               => KEY_10,
			&Key::Minus            => KEY_MINUS,
			&Key::Equal            => KEY_EQUAL,
			&Key::BackSpace        => KEY_BACKSPACE,
			&Key::Tab              => KEY_TAB,
			&Key::Q                => KEY_Q,
			&Key::W                => KEY_W,
			&Key::E                => KEY_E,
			&Key::R                => KEY_R,
			&Key::T                => KEY_T,
			&Key::Y                => KEY_Y,
			&Key::U                => KEY_U,
			&Key::I                => KEY_I,
			&Key::O                => KEY_O,
			&Key::P                => KEY_P,
			&Key::LeftBrace        => KEY_LEFTBRACE,
			&Key::RightBrace       => KEY_RIGHTBRACE,
			&Key::Enter            => KEY_ENTER,
			&Key::LeftControl      => KEY_LEFTCTRL,
			&Key::A                => KEY_A,
			&Key::S                => KEY_S,
			&Key::D                => KEY_D,
			&Key::F                => KEY_F,
			&Key::G                => KEY_G,
			&Key::H                => KEY_H,
			&Key::J                => KEY_J,
			&Key::K                => KEY_K,
			&Key::L                => KEY_L,
			&Key::SemiColon        => KEY_SEMICOLON,
			&Key::Apostrophe       => KEY_APOSTROPHE,
			&Key::Grave            => KEY_GRAVE,
			&Key::LeftShift        => KEY_LEFTSHIFT,
			&Key::BackSlash        => KEY_BACKSLASH,
			&Key::Z                => KEY_Z,
			&Key::X                => KEY_X,
			&Key::C                => KEY_C,
			&Key::V                => KEY_V,
			&Key::B                => KEY_B,
			&Key::N                => KEY_N,
			&Key::M                => KEY_M,
			&Key::Comma            => KEY_COMMA,
			&Key::Dot              => KEY_DOT,
			&Key::Slash            => KEY_SLASH,
			&Key::RightShift       => KEY_RIGHTSHIFT,
			&Key::LeftAlt          => KEY_LEFTALT,
			&Key::Space            => KEY_SPACE,
			&Key::CapsLock         => KEY_CAPSLOCK,
			&Key::F1               => KEY_F1,
			&Key::F2               => KEY_F2,
			&Key::F3               => KEY_F3,
			&Key::F4               => KEY_F4,
			&Key::F5               => KEY_F5,
			&Key::F6               => KEY_F6,
			&Key::F7               => KEY_F7,
			&Key::F8               => KEY_F8,
			&Key::F9               => KEY_F9,
			&Key::F10              => KEY_F10,
			&Key::NumLock          => KEY_NUMLOCK,
			&Key::ScrollLock       => KEY_SCROLLLOCK,
			&Key::F11              => KEY_F11,
			&Key::F12              => KEY_F12,
			&Key::RightControl     => KEY_RIGHTCTRL,
			&Key::SysRq            => KEY_SYSRQ,
			&Key::RightAlt         => KEY_RIGHTALT,
			&Key::LineFeed         => KEY_LINEFEED,
			&Key::Home             => KEY_HOME,
			&Key::Up               => KEY_UP,
			&Key::PageUp           => KEY_PAGEUP,
			&Key::Left             => KEY_LEFT,
			&Key::Right            => KEY_RIGHT,
			&Key::End              => KEY_END,
			&Key::Down             => KEY_DOWN,
			&Key::PageDown         => KEY_PAGEDOWN,
			&Key::Insert           => KEY_INSERT,
			&Key::Delete           => KEY_DELETE,
			&Key::LeftMeta         => KEY_LEFTMETA,
			&Key::RightMeta        => KEY_RIGHTMETA,
			&Key::ScrollUp         => KEY_SCROLLUP,
			&Key::ScrollDown       => KEY_SCROLLDOWN,
			&Key::F13              => KEY_F13,
			&Key::F14              => KEY_F14,
			&Key::F15              => KEY_F15,
			&Key::F16              => KEY_F16,
			&Key::F17              => KEY_F17,
			&Key::F18              => KEY_F18,
			&Key::F19              => KEY_F19,
			&Key::F20              => KEY_F20,
			&Key::F21              => KEY_F21,
			&Key::F22              => KEY_F22,
			&Key::F23              => KEY_F23,
			&Key::F24              => KEY_F24,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(KeyPadVariants))]
	pub enum KeyPad {
		Asterisk,
		_7,
		_8,
		_9,
		Minus,
		_4,
		_5,
		_6,
		Plus,
		_1,
		_2,
		_3,
		_0,
		Dot,
		AltComma,
		Enter,
		Slash,
		Equal,
		PlusMinus,
		Comma,
		LeftParen,
		RightParen,
	}
}

impl Into<Event> for KeyPad {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::KeyPad(self))
	}
}

impl Press for KeyPad { }
impl Release for KeyPad { }

impl Kind for KeyPad {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for KeyPad {
	fn code(&self) -> c_int {
		match self {
			&KeyPad::Asterisk   => KEY_KPASTERISK,
			&KeyPad::_7         => KEY_KP7,
			&KeyPad::_8         => KEY_KP8,
			&KeyPad::_9         => KEY_KP9,
			&KeyPad::Minus      => KEY_KPMINUS,
			&KeyPad::_4         => KEY_KP4,
			&KeyPad::_5         => KEY_KP5,
			&KeyPad::_6         => KEY_KP6,
			&KeyPad::Plus       => KEY_KPPLUS,
			&KeyPad::_1         => KEY_KP1,
			&KeyPad::_2         => KEY_KP2,
			&KeyPad::_3         => KEY_KP3,
			&KeyPad::_0         => KEY_KP0,
			&KeyPad::Dot        => KEY_KPDOT,
			&KeyPad::AltComma   => KEY_KPJPCOMMA,
			&KeyPad::Enter      => KEY_KPENTER,
			&KeyPad::Slash      => KEY_KPSLASH,
			&KeyPad::Equal      => KEY_KPEQUAL,
			&KeyPad::PlusMinus  => KEY_KPPLUSMINUS,
			&KeyPad::Comma      => KEY_KPCOMMA,
			&KeyPad::LeftParen  => KEY_KPLEFTPAREN,
			&KeyPad::RightParen => KEY_KPRIGHTPAREN,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(MiscVariants))]
	pub enum Misc {
		ZenkakuHankaku,
		ND102,
		RO,
		Katakana,
		Hiragana,
		Henkan,
		KatakanaHiragana,
		Muhenkan,
		Macro,
		Mute,
		VolumeDown,
		VolumeUp,
		Power,
		Pause,
		Scale,
		Hangeul,
		Hanguel,
		Hanja,
		Yen,
		Compose,
		Stop,
		Again,
		Props,
		Undo,
		Front,
		Copy,
		Open,
		Paste,
		Find,
		Cut,
		Help,
		Menu,
		Calc,
		Setup,
		Sleep,
		WakeUp,
		File,
		SendFile,
		DeleteFile,
		XFer,
		Prog1,
		Prog2,
		WWW,
		MSDOS,
		Coffee,
		ScreenLock,
		RotateDisplay,
		Direction,
		CycleWindows,
		Mail,
		Bookmarks,
		Computer,
		Back,
		Forward,
		CloseCD,
		EjectCD,
		EjectCloseCD,
		NextSong,
		PlayPause,
		PreviousSong,
		StopCD,
		Record,
		Rewind,
		Phone,
		Iso,
		Config,
		HomePage,
		Refresh,
		Exit,
		Move,
		Edit,
		New,
		Redo,
		PlayCD,
		PauseCD,
		Prog3,
		Prog4,
		DashBoard,
		Suspend,
		Close,
		Play,
		FastForward,
		BassBoost,
		Print,
		HP,
		Camera,
		Sound,
		Question,
		Email,
		Chat,
		Search,
		Connect,
		Finance,
		Sport,
		Shop,
		AltErase,
		Cancel,
		BrightnessDown,
		BrightnessUp,
		Media,
		SwitchVideoMode,
		IllumToggle,
		IllumDown,
		IllumUp,
		Send,
		Reply,
		ForwardEmail,
		Save,
		Documents,
		Battery,
		Bluetooth,
		WLAN,
		UWB,
		Unknown,
		VideoNext,
		VideoPrev,
		BrightnessCycle,
		BrightnessAuto,
		BrightnessZero,
		DisplayOff,
		WWAN,
		WIMAX,
		RFKILL,
		MicMute,
		Ok,
		Select,
		Goto,
		Clear,
		Power2,
		Option,
		Info,
		Time,
		Vendor,
		Archive,
		Program,
		Channel,
		Favorites,
		EPG,
		PVR,
		MHP,
		Language,
		Title,
		Subtitle,
		Angle,
		Zoom,
		Mode,
		Keyboard,
		Screen,
		PC,
		TV,
		TV2,
		VCR,
		VCR2,
		SAT,
		SAT2,
		CD,
		Tape,
		Radio,
		Tuner,
		Player,
		Text,
		DVD,
		AUX,
		MP3,
		Audio,
		Video,
		Directory,
		List,
		Memo,
		Calendar,
		Red,
		Green,
		Yellow,
		Blue,
		ChannelUp,
		ChannelDown,
		First,
		Last,
		AB,
		Next,
		Restart,
		Slow,
		Shuffle,
		Break,
		Previous,
		Digits,
		TEEN,
		TWEN,
		VideoPhone,
		Games,
		ZoomIn,
		ZoomOut,
		ZoomReset,
		WordProcessor,
		Editor,
		SpreadSheet,
		GraphicsEditor,
		Presentation,
		Database,
		News,
		VoiceMail,
		AddressBook,
		Messenger,
		DisplayToggle,
		BrightnessToggle,
		SpellCheck,
		LogOff,
		Dollar,
		Euro,
		FrameBack,
		FrameForward,
		ContextMenu,
		MediaRepeat,
		Up10Channels,
		Down10Channels,
		Images,
		DeleteEOL,
		DeleteEOS,
		InsertLine,
		DeleteLine,
		WPS,
		LightsToggle,
		ALSToggle,
		ButtonConfig,
		TaskManager,
		Journal,
		ControlPanel,
		AppSelect,
		ScreenSaver,
		VoiceCommand,
		BrighnessMin,
		BrightnessMax,
	}
}

impl Into<Event> for Misc {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Misc(self))
	}
}

impl Press for Misc { }
impl Release for Misc { }

impl Kind for Misc {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Misc {
	fn code(&self) -> c_int {
		match self {
			&Misc::ZenkakuHankaku   => KEY_ZENKAKUHANKAKU,
			&Misc::ND102            => KEY_102ND,
			&Misc::RO               => KEY_RO,
			&Misc::Katakana         => KEY_KATAKANA,
			&Misc::Hiragana         => KEY_HIRAGANA,
			&Misc::Henkan           => KEY_HENKAN,
			&Misc::KatakanaHiragana => KEY_KATAKANAHIRAGANA,
			&Misc::Muhenkan         => KEY_MUHENKAN,
			&Misc::Macro            => KEY_MACRO,
			&Misc::Mute             => KEY_MUTE,
			&Misc::VolumeDown       => KEY_VOLUMEDOWN,
			&Misc::VolumeUp         => KEY_VOLUMEUP,
			&Misc::Power            => KEY_POWER,
			&Misc::Pause            => KEY_PAUSE,
			&Misc::Scale            => KEY_SCALE,
			&Misc::Hangeul          => KEY_HANGEUL,
			&Misc::Hanguel          => KEY_HANGUEL,
			&Misc::Hanja            => KEY_HANJA,
			&Misc::Yen              => KEY_YEN,
			&Misc::Compose          => KEY_COMPOSE,
			&Misc::Stop             => KEY_STOP,
			&Misc::Again            => KEY_AGAIN,
			&Misc::Props            => KEY_PROPS,
			&Misc::Undo             => KEY_UNDO,
			&Misc::Front            => KEY_FRONT,
			&Misc::Copy             => KEY_COPY,
			&Misc::Open             => KEY_OPEN,
			&Misc::Paste            => KEY_PASTE,
			&Misc::Find             => KEY_FIND,
			&Misc::Cut              => KEY_CUT,
			&Misc::Help             => KEY_HELP,
			&Misc::Menu             => KEY_MENU,
			&Misc::Calc             => KEY_CALC,
			&Misc::Setup            => KEY_SETUP,
			&Misc::Sleep            => KEY_SLEEP,
			&Misc::WakeUp           => KEY_WAKEUP,
			&Misc::File             => KEY_FILE,
			&Misc::SendFile         => KEY_SENDFILE,
			&Misc::DeleteFile       => KEY_DELETEFILE,
			&Misc::XFer             => KEY_XFER,
			&Misc::Prog1            => KEY_PROG1,
			&Misc::Prog2            => KEY_PROG2,
			&Misc::WWW              => KEY_WWW,
			&Misc::MSDOS            => KEY_MSDOS,
			&Misc::Coffee           => KEY_COFFEE,
			&Misc::ScreenLock       => KEY_SCREENLOCK,
			&Misc::RotateDisplay    => KEY_ROTATE_DISPLAY,
			&Misc::Direction        => KEY_DIRECTION,
			&Misc::CycleWindows     => KEY_CYCLEWINDOWS,
			&Misc::Mail             => KEY_MAIL,
			&Misc::Bookmarks        => KEY_BOOKMARKS,
			&Misc::Computer         => KEY_COMPUTER,
			&Misc::Back             => KEY_BACK,
			&Misc::Forward          => KEY_FORWARD,
			&Misc::CloseCD          => KEY_CLOSECD,
			&Misc::EjectCD          => KEY_EJECTCD,
			&Misc::EjectCloseCD     => KEY_EJECTCLOSECD,
			&Misc::NextSong         => KEY_NEXTSONG,
			&Misc::PlayPause        => KEY_PLAYPAUSE,
			&Misc::PreviousSong     => KEY_PREVIOUSSONG,
			&Misc::StopCD           => KEY_STOPCD,
			&Misc::Record           => KEY_RECORD,
			&Misc::Rewind           => KEY_REWIND,
			&Misc::Phone            => KEY_PHONE,
			&Misc::Iso              => KEY_ISO,
			&Misc::Config           => KEY_CONFIG,
			&Misc::HomePage         => KEY_HOMEPAGE,
			&Misc::Refresh          => KEY_REFRESH,
			&Misc::Exit             => KEY_EXIT,
			&Misc::Move             => KEY_MOVE,
			&Misc::Edit             => KEY_EDIT,
			&Misc::New              => KEY_NEW,
			&Misc::Redo             => KEY_REDO,
			&Misc::PlayCD           => KEY_PLAYCD,
			&Misc::PauseCD          => KEY_PAUSECD,
			&Misc::Prog3            => KEY_PROG3,
			&Misc::Prog4            => KEY_PROG4,
			&Misc::DashBoard        => KEY_DASHBOARD,
			&Misc::Suspend          => KEY_SUSPEND,
			&Misc::Close            => KEY_CLOSE,
			&Misc::Play             => KEY_PLAY,
			&Misc::FastForward      => KEY_FASTFORWARD,
			&Misc::BassBoost        => KEY_BASSBOOST,
			&Misc::Print            => KEY_PRINT,
			&Misc::HP               => KEY_HP,
			&Misc::Camera           => KEY_CAMERA,
			&Misc::Sound            => KEY_SOUND,
			&Misc::Question         => KEY_QUESTION,
			&Misc::Email            => KEY_EMAIL,
			&Misc::Chat             => KEY_CHAT,
			&Misc::Search           => KEY_SEARCH,
			&Misc::Connect          => KEY_CONNECT,
			&Misc::Finance          => KEY_FINANCE,
			&Misc::Sport            => KEY_SPORT,
			&Misc::Shop             => KEY_SHOP,
			&Misc::AltErase         => KEY_ALTERASE,
			&Misc::Cancel           => KEY_CANCEL,
			&Misc::BrightnessDown   => KEY_BRIGHTNESSDOWN,
			&Misc::BrightnessUp     => KEY_BRIGHTNESSUP,
			&Misc::Media            => KEY_MEDIA,
			&Misc::SwitchVideoMode  => KEY_SWITCHVIDEOMODE,
			&Misc::IllumToggle      => KEY_KBDILLUMTOGGLE,
			&Misc::IllumDown        => KEY_KBDILLUMDOWN,
			&Misc::IllumUp          => KEY_KBDILLUMUP,
			&Misc::Send             => KEY_SEND,
			&Misc::Reply            => KEY_REPLY,
			&Misc::ForwardEmail     => KEY_FORWARDMAIL,
			&Misc::Save             => KEY_SAVE,
			&Misc::Documents        => KEY_DOCUMENTS,
			&Misc::Battery          => KEY_BATTERY,
			&Misc::Bluetooth        => KEY_BLUETOOTH,
			&Misc::WLAN             => KEY_WLAN,
			&Misc::UWB              => KEY_UWB,
			&Misc::Unknown          => KEY_UNKNOWN,
			&Misc::VideoNext        => KEY_VIDEO_NEXT,
			&Misc::VideoPrev        => KEY_VIDEO_PREV,
			&Misc::BrightnessCycle  => KEY_BRIGHTNESS_CYCLE,
			&Misc::BrightnessAuto   => KEY_BRIGHTNESS_AUTO,
			&Misc::BrightnessZero   => KEY_BRIGHTNESS_ZERO,
			&Misc::DisplayOff       => KEY_DISPLAY_OFF,
			&Misc::WWAN             => KEY_WWAN,
			&Misc::WIMAX            => KEY_WIMAX,
			&Misc::RFKILL           => KEY_RFKILL,
			&Misc::MicMute          => KEY_MICMUTE,
			&Misc::Ok               => KEY_OK,
			&Misc::Select           => KEY_SELECT,
			&Misc::Goto             => KEY_GOTO,
			&Misc::Clear            => KEY_CLEAR,
			&Misc::Power2           => KEY_POWER2,
			&Misc::Option           => KEY_OPTION,
			&Misc::Info             => KEY_INFO,
			&Misc::Time             => KEY_TIME,
			&Misc::Vendor           => KEY_VENDOR,
			&Misc::Archive          => KEY_ARCHIVE,
			&Misc::Program          => KEY_PROGRAM,
			&Misc::Channel          => KEY_CHANNEL,
			&Misc::Favorites        => KEY_FAVORITES,
			&Misc::EPG              => KEY_EPG,
			&Misc::PVR              => KEY_PVR,
			&Misc::MHP              => KEY_MHP,
			&Misc::Language         => KEY_LANGUAGE,
			&Misc::Title            => KEY_TITLE,
			&Misc::Subtitle         => KEY_SUBTITLE,
			&Misc::Angle            => KEY_ANGLE,
			&Misc::Zoom             => KEY_ZOOM,
			&Misc::Mode             => KEY_MODE,
			&Misc::Keyboard         => KEY_KEYBOARD,
			&Misc::Screen           => KEY_SCREEN,
			&Misc::PC               => KEY_PC,
			&Misc::TV               => KEY_TV,
			&Misc::TV2              => KEY_TV2,
			&Misc::VCR              => KEY_VCR,
			&Misc::VCR2             => KEY_VCR2,
			&Misc::SAT              => KEY_SAT,
			&Misc::SAT2             => KEY_SAT2,
			&Misc::CD               => KEY_CD,
			&Misc::Tape             => KEY_TAPE,
			&Misc::Radio            => KEY_RADIO,
			&Misc::Tuner            => KEY_TUNER,
			&Misc::Player           => KEY_PLAYER,
			&Misc::Text             => KEY_TEXT,
			&Misc::DVD              => KEY_DVD,
			&Misc::AUX              => KEY_AUX,
			&Misc::MP3              => KEY_MP3,
			&Misc::Audio            => KEY_AUDIO,
			&Misc::Video            => KEY_VIDEO,
			&Misc::Directory        => KEY_DIRECTORY,
			&Misc::List             => KEY_LIST,
			&Misc::Memo             => KEY_MEMO,
			&Misc::Calendar         => KEY_CALENDAR,
			&Misc::Red              => KEY_RED,
			&Misc::Green            => KEY_GREEN,
			&Misc::Yellow           => KEY_YELLOW,
			&Misc::Blue             => KEY_BLUE,
			&Misc::ChannelUp        => KEY_CHANNELUP,
			&Misc::ChannelDown      => KEY_CHANNELDOWN,
			&Misc::First            => KEY_FIRST,
			&Misc::Last             => KEY_LAST,
			&Misc::AB               => KEY_AB,
			&Misc::Next             => KEY_NEXT,
			&Misc::Restart          => KEY_RESTART,
			&Misc::Slow             => KEY_SLOW,
			&Misc::Shuffle          => KEY_SHUFFLE,
			&Misc::Break            => KEY_BREAK,
			&Misc::Previous         => KEY_PREVIOUS,
			&Misc::Digits           => KEY_DIGITS,
			&Misc::TEEN             => KEY_TEEN,
			&Misc::TWEN             => KEY_TWEN,
			&Misc::VideoPhone       => KEY_VIDEOPHONE,
			&Misc::Games            => KEY_GAMES,
			&Misc::ZoomIn           => KEY_ZOOMIN,
			&Misc::ZoomOut          => KEY_ZOOMOUT,
			&Misc::ZoomReset        => KEY_ZOOMRESET,
			&Misc::WordProcessor    => KEY_WORDPROCESSOR,
			&Misc::Editor           => KEY_EDITOR,
			&Misc::SpreadSheet      => KEY_SPREADSHEET,
			&Misc::GraphicsEditor   => KEY_GRAPHICSEDITOR,
			&Misc::Presentation     => KEY_PRESENTATION,
			&Misc::Database         => KEY_DATABASE,
			&Misc::News             => KEY_NEWS,
			&Misc::VoiceMail        => KEY_VOICEMAIL,
			&Misc::AddressBook      => KEY_ADDRESSBOOK,
			&Misc::Messenger        => KEY_MESSENGER,
			&Misc::DisplayToggle    => KEY_DISPLAYTOGGLE,
			&Misc::BrightnessToggle => KEY_BRIGHTNESS_TOGGLE,
			&Misc::SpellCheck       => KEY_SPELLCHECK,
			&Misc::LogOff           => KEY_LOGOFF,
			&Misc::Dollar           => KEY_DOLLAR,
			&Misc::Euro             => KEY_EURO,
			&Misc::FrameBack        => KEY_FRAMEBACK,
			&Misc::FrameForward     => KEY_FRAMEFORWARD,
			&Misc::ContextMenu      => KEY_CONTEXT_MENU,
			&Misc::MediaRepeat      => KEY_MEDIA_REPEAT,
			&Misc::Up10Channels     => KEY_10CHANNELSUP,
			&Misc::Down10Channels   => KEY_10CHANNELSDOWN,
			&Misc::Images           => KEY_IMAGES,
			&Misc::DeleteEOL        => KEY_DEL_EOL,
			&Misc::DeleteEOS        => KEY_DEL_EOS,
			&Misc::InsertLine       => KEY_INS_LINE,
			&Misc::DeleteLine       => KEY_DEL_LINE,
			&Misc::WPS              => KEY_WPS_BUTTON,
			&Misc::LightsToggle     => KEY_LIGHTS_TOGGLE,
			&Misc::ALSToggle        => KEY_ALS_TOGGLE,
			&Misc::ButtonConfig     => KEY_BUTTONCONFIG,
			&Misc::TaskManager      => KEY_TASKMANAGER,
			&Misc::Journal          => KEY_JOURNAL,
			&Misc::ControlPanel     => KEY_CONTROLPANEL,
			&Misc::AppSelect        => KEY_APPSELECT,
			&Misc::ScreenSaver      => KEY_SCREENSAVER,
			&Misc::VoiceCommand     => KEY_VOICECOMMAND,
			&Misc::BrighnessMin     => KEY_BRIGHTNESS_MIN,
			&Misc::BrightnessMax    => KEY_BRIGHTNESS_MAX,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(FunctionVariants))]
	pub enum Function {
		Press,
		Esc,
		F1,
		F2,
		F3,
		F4,
		F5,
		F6,
		F7,
		F8,
		F9,
		F10,
		F11,
		F12,
		_1,
		_2,
		D,
		E,
		F,
		S,
		B,
	}
}

impl Into<Event> for Function {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Function(self))
	}
}

impl Press for Function { }
impl Release for Function { }

impl Kind for Function {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Function {
	fn code(&self) -> c_int {
		match self {
			&Function::Press => KEY_FN,
			&Function::Esc   => KEY_FN_ESC,
			&Function::F1    => KEY_FN_F1,
			&Function::F2    => KEY_FN_F2,
			&Function::F3    => KEY_FN_F3,
			&Function::F4    => KEY_FN_F4,
			&Function::F5    => KEY_FN_F5,
			&Function::F6    => KEY_FN_F6,
			&Function::F7    => KEY_FN_F7,
			&Function::F8    => KEY_FN_F8,
			&Function::F9    => KEY_FN_F9,
			&Function::F10   => KEY_FN_F10,
			&Function::F11   => KEY_FN_F11,
			&Function::F12   => KEY_FN_F12,
			&Function::_1    => KEY_FN_1,
			&Function::_2    => KEY_FN_2,
			&Function::D     => KEY_FN_D,
			&Function::E     => KEY_FN_E,
			&Function::F     => KEY_FN_F,
			&Function::S     => KEY_FN_S,
			&Function::B     => KEY_FN_B,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(BrailleVariants))]
	pub enum Braille {
		Dot1,
		Dot2,
		Dot3,
		Dot4,
		Dot5,
		Dot6,
		Dot7,
		Dot8,
		Dot9,
		Dot10,
	}
}

impl Into<Event> for Braille {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Braille(self))
	}
}

impl Press for Braille { }
impl Release for Braille { }

impl Kind for Braille {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Braille {
	fn code(&self) -> c_int {
		match self {
			&Braille::Dot1  => KEY_BRL_DOT1,
			&Braille::Dot2  => KEY_BRL_DOT2,
			&Braille::Dot3  => KEY_BRL_DOT3,
			&Braille::Dot4  => KEY_BRL_DOT4,
			&Braille::Dot5  => KEY_BRL_DOT5,
			&Braille::Dot6  => KEY_BRL_DOT6,
			&Braille::Dot7  => KEY_BRL_DOT7,
			&Braille::Dot8  => KEY_BRL_DOT8,
			&Braille::Dot9  => KEY_BRL_DOT9,
			&Braille::Dot10 => KEY_BRL_DOT10,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(NumericVariants))]
	pub enum Numeric {
		_0,
		_1,
		_2,
		_3,
		_4,
		_5,
		_6,
		_7,
		_8,
		_9,
		Star,
		Pound,
		A,
		B,
		C,
		D,
	}
}

impl Into<Event> for Numeric {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Numeric(self))
	}
}

impl Press for Numeric { }
impl Release for Numeric { }

impl Kind for Numeric {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Numeric {
	fn code(&self) -> c_int {
		match self {
			&Numeric::_0    => KEY_NUMERIC_0,
			&Numeric::_1    => KEY_NUMERIC_1,
			&Numeric::_2    => KEY_NUMERIC_2,
			&Numeric::_3    => KEY_NUMERIC_3,
			&Numeric::_4    => KEY_NUMERIC_4,
			&Numeric::_5    => KEY_NUMERIC_5,
			&Numeric::_6    => KEY_NUMERIC_6,
			&Numeric::_7    => KEY_NUMERIC_7,
			&Numeric::_8    => KEY_NUMERIC_8,
			&Numeric::_9    => KEY_NUMERIC_9,
			&Numeric::Star  => KEY_NUMERIC_STAR,
			&Numeric::Pound => KEY_NUMERIC_POUND,
			&Numeric::A     => KEY_NUMERIC_A,
			&Numeric::B     => KEY_NUMERIC_B,
			&Numeric::C     => KEY_NUMERIC_C,
			&Numeric::D     => KEY_NUMERIC_D,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(TouchPadVariants))]
	pub enum TouchPad {
		Toggle,
		On,
		Off,
	}
}

impl Into<Event> for TouchPad {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::TouchPad(self))
	}
}

impl Press for TouchPad { }
impl Release for TouchPad { }

impl Kind for TouchPad {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for TouchPad {
	fn code(&self) -> c_int {
		match self {
			&TouchPad::Toggle => KEY_TOUCHPAD_TOGGLE,
			&TouchPad::On     => KEY_TOUCHPAD_ON,
			&TouchPad::Off    => KEY_TOUCHPAD_OFF,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(CameraVariants))]
	pub enum Camera {
		ZoomIn,
		ZoomOut,
		Up,
		Down,
		Left,
		Right,
		Focus,
	}
}

impl Into<Event> for Camera {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Camera(self))
	}
}

impl Press for Camera { }
impl Release for Camera { }

impl Kind for Camera {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Camera {
	fn code(&self) -> c_int {
		match self {
			&Camera::ZoomIn  => KEY_CAMERA_ZOOMIN,
			&Camera::ZoomOut => KEY_CAMERA_ZOOMOUT,
			&Camera::Up      => KEY_CAMERA_UP,
			&Camera::Down    => KEY_CAMERA_DOWN,
			&Camera::Left    => KEY_CAMERA_LEFT,
			&Camera::Right   => KEY_CAMERA_RIGHT,
			&Camera::Focus   => KEY_CAMERA_FOCUS,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(AttendantVariants))]
	pub enum Attendant {
		On,
		Off,
		Toggle,
	}
}

impl Into<Event> for Attendant {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::Attendant(self))
	}
}

impl Press for Attendant { }
impl Release for Attendant { }

impl Kind for Attendant {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for Attendant {
	fn code(&self) -> c_int {
		match self {
			&Attendant::On     => KEY_ATTENDANT_ON,
			&Attendant::Off    => KEY_ATTENDANT_OFF,
			&Attendant::Toggle => KEY_ATTENDANT_TOGGLE,
		}
	}
}

custom_derive! {
	#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, IterVariants(InputAssistVariants))]
	pub enum InputAssist {
		Prev,
		Next,
		PrevGroup,
		NextGroup,
		Accept,
		Cancel,
	}
}

impl Into<Event> for InputAssist {
	fn into(self) -> Event {
		Event::Keyboard(Keyboard::InputAssist(self))
	}
}

impl Press for InputAssist { }
impl Release for InputAssist { }

impl Kind for InputAssist {
	fn kind(&self) -> c_int {
		EV_KEY
	}
}

impl Code for InputAssist {
	fn code(&self) -> c_int {
		match self {
			&InputAssist::Prev      => KEY_KBDINPUTASSIST_PREV,
			&InputAssist::Next      => KEY_KBDINPUTASSIST_NEXT,
			&InputAssist::PrevGroup => KEY_KBDINPUTASSIST_PREVGROUP,
			&InputAssist::NextGroup => KEY_KBDINPUTASSIST_NEXTGROUP,
			&InputAssist::Accept    => KEY_KBDINPUTASSIST_ACCEPT,
			&InputAssist::Cancel    => KEY_KBDINPUTASSIST_CANCEL,
		}
	}
}
