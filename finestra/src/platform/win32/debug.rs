// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

pub const WINDOW_MESSAGE_BY_NAME: &[(&'static str, u32)] = &[
    ("WM_ACTIVATE", 6u32),
    ("WM_ACTIVATEAPP", 28u32),
    ("WM_AFXFIRST", 864u32),
    ("WM_AFXLAST", 895u32),
    ("WM_APP", 32768u32),
    ("WM_APPCOMMAND", 793u32),
    ("WM_ASKCBFORMATNAME", 780u32),
    ("WM_CANCELJOURNAL", 75u32),
    ("WM_CANCELMODE", 31u32),
    ("WM_CAPTURECHANGED", 533u32),
    ("WM_CHANGECBCHAIN", 781u32),
    ("WM_CHANGEUISTATE", 295u32),
    ("WM_CHAR", 258u32),
    ("WM_CHARTOITEM", 47u32),
    ("WM_CHILDACTIVATE", 34u32),
    ("WM_CLEAR", 771u32),
    ("WM_CLIPBOARDUPDATE", 797u32),
    ("WM_CLOSE", 16u32),
    ("WM_COMMAND", 273u32),
    ("WM_COMMNOTIFY", 68u32),
    ("WM_COMPACTING", 65u32),
    ("WM_COMPAREITEM", 57u32),
    ("WM_CONTEXTMENU", 123u32),
    ("WM_COPY", 769u32),
    ("WM_COPYDATA", 74u32),
    ("WM_CREATE", 1u32),
    ("WM_CTLCOLORBTN", 309u32),
    ("WM_CTLCOLORDLG", 310u32),
    ("WM_CTLCOLOREDIT", 307u32),
    ("WM_CTLCOLORLISTBOX", 308u32),
    ("WM_CTLCOLORMSGBOX", 306u32),
    ("WM_CTLCOLORSCROLLBAR", 311u32),
    ("WM_CTLCOLORSTATIC", 312u32),
    ("WM_CUT", 768u32),
    ("WM_DEADCHAR", 259u32),
    ("WM_DELETEITEM", 45u32),
    ("WM_DESTROY", 2u32),
    ("WM_DESTROYCLIPBOARD", 775u32),
    ("WM_DEVICECHANGE", 537u32),
    ("WM_DEVMODECHANGE", 27u32),
    ("WM_DISPLAYCHANGE", 126u32),
    ("WM_DPICHANGED", 736u32),
    ("WM_DPICHANGED_AFTERPARENT", 739u32),
    ("WM_DPICHANGED_BEFOREPARENT", 738u32),
    ("WM_DRAWCLIPBOARD", 776u32),
    ("WM_DRAWITEM", 43u32),
    ("WM_DROPFILES", 563u32),
    ("WM_DWMCOLORIZATIONCOLORCHANGED", 800u32),
    ("WM_DWMCOMPOSITIONCHANGED", 798u32),
    ("WM_DWMNCRENDERINGCHANGED", 799u32),
    ("WM_DWMSENDICONICLIVEPREVIEWBITMAP", 806u32),
    ("WM_DWMSENDICONICTHUMBNAIL", 803u32),
    ("WM_DWMWINDOWMAXIMIZEDCHANGE", 801u32),
    ("WM_ENABLE", 10u32),
    ("WM_ENDSESSION", 22u32),
    ("WM_ENTERIDLE", 289u32),
    ("WM_ENTERMENULOOP", 529u32),
    ("WM_ENTERSIZEMOVE", 561u32),
    ("WM_ERASEBKGND", 20u32),
    ("WM_EXITMENULOOP", 530u32),
    ("WM_EXITSIZEMOVE", 562u32),
    ("WM_FONTCHANGE", 29u32),
    ("WM_GESTURE", 281u32),
    ("WM_GESTURENOTIFY", 282u32),
    ("WM_GETDLGCODE", 135u32),
    ("WM_GETDPISCALEDSIZE", 740u32),
    ("WM_GETFONT", 49u32),
    ("WM_GETHOTKEY", 51u32),
    ("WM_GETICON", 127u32),
    ("WM_GETMINMAXINFO", 36u32),
    ("WM_GETOBJECT", 61u32),
    ("WM_GETTEXT", 13u32),
    ("WM_GETTEXTLENGTH", 14u32),
    ("WM_GETTITLEBARINFOEX", 831u32),
    ("WM_HANDHELDFIRST", 856u32),
    ("WM_HANDHELDLAST", 863u32),
    ("WM_HELP", 83u32),
    ("WM_HOTKEY", 786u32),
    ("WM_HSCROLL", 276u32),
    ("WM_HSCROLLCLIPBOARD", 782u32),
    ("WM_ICONERASEBKGND", 39u32),
    ("WM_IME_CHAR", 646u32),
    ("WM_IME_COMPOSITION", 271u32),
    ("WM_IME_COMPOSITIONFULL", 644u32),
    ("WM_IME_CONTROL", 643u32),
    ("WM_IME_ENDCOMPOSITION", 270u32),
    ("WM_IME_KEYDOWN", 656u32),
    ("WM_IME_KEYLAST", 271u32),
    ("WM_IME_KEYUP", 657u32),
    ("WM_IME_NOTIFY", 642u32),
    ("WM_IME_REQUEST", 648u32),
    ("WM_IME_SELECT", 645u32),
    ("WM_IME_SETCONTEXT", 641u32),
    ("WM_IME_STARTCOMPOSITION", 269u32),
    ("WM_INITDIALOG", 272u32),
    ("WM_INITMENU", 278u32),
    ("WM_INITMENUPOPUP", 279u32),
    ("WM_INPUT", 255u32),
    ("WM_INPUTLANGCHANGE", 81u32),
    ("WM_INPUTLANGCHANGEREQUEST", 80u32),
    ("WM_INPUT_DEVICE_CHANGE", 254u32),
    ("WM_KEYDOWN", 256u32),
    ("WM_KEYFIRST", 256u32),
    ("WM_KEYLAST", 265u32),
    ("WM_KEYUP", 257u32),
    ("WM_KILLFOCUS", 8u32),
    ("WM_LBUTTONDBLCLK", 515u32),
    ("WM_LBUTTONDOWN", 513u32),
    ("WM_LBUTTONUP", 514u32),
    ("WM_MBUTTONDBLCLK", 521u32),
    ("WM_MBUTTONDOWN", 519u32),
    ("WM_MBUTTONUP", 520u32),
    ("WM_MDIACTIVATE", 546u32),
    ("WM_MDICASCADE", 551u32),
    ("WM_MDICREATE", 544u32),
    ("WM_MDIDESTROY", 545u32),
    ("WM_MDIGETACTIVE", 553u32),
    ("WM_MDIICONARRANGE", 552u32),
    ("WM_MDIMAXIMIZE", 549u32),
    ("WM_MDINEXT", 548u32),
    ("WM_MDIREFRESHMENU", 564u32),
    ("WM_MDIRESTORE", 547u32),
    ("WM_MDISETMENU", 560u32),
    ("WM_MDITILE", 550u32),
    ("WM_MEASUREITEM", 44u32),
    ("WM_MENUCHAR", 288u32),
    ("WM_MENUCOMMAND", 294u32),
    ("WM_MENUDRAG", 291u32),
    ("WM_MENUGETOBJECT", 292u32),
    ("WM_MENURBUTTONUP", 290u32),
    ("WM_MENUSELECT", 287u32),
    ("WM_MOUSEACTIVATE", 33u32),
    ("WM_MOUSEFIRST", 512u32),
    ("WM_MOUSEHWHEEL", 526u32),
    ("WM_MOUSELAST", 526u32),
    ("WM_MOUSEMOVE", 512u32),
    ("WM_MOUSEWHEEL", 522u32),
    ("WM_MOVE", 3u32),
    ("WM_MOVING", 534u32),
    ("WM_NCACTIVATE", 134u32),
    ("WM_NCCALCSIZE", 131u32),
    ("WM_NCCREATE", 129u32),
    ("WM_NCDESTROY", 130u32),
    ("WM_NCHITTEST", 132u32),
    ("WM_NCLBUTTONDBLCLK", 163u32),
    ("WM_NCLBUTTONDOWN", 161u32),
    ("WM_NCLBUTTONUP", 162u32),
    ("WM_NCMBUTTONDBLCLK", 169u32),
    ("WM_NCMBUTTONDOWN", 167u32),
    ("WM_NCMBUTTONUP", 168u32),
    ("WM_NCMOUSEHOVER", 672u32),
    ("WM_NCMOUSELEAVE", 674u32),
    ("WM_NCMOUSEMOVE", 160u32),
    ("WM_NCPAINT", 133u32),
    ("WM_NCPOINTERDOWN", 578u32),
    ("WM_NCPOINTERUP", 579u32),
    ("WM_NCPOINTERUPDATE", 577u32),
    ("WM_NCRBUTTONDBLCLK", 166u32),
    ("WM_NCRBUTTONDOWN", 164u32),
    ("WM_NCRBUTTONUP", 165u32),
    ("WM_NCXBUTTONDBLCLK", 173u32),
    ("WM_NCXBUTTONDOWN", 171u32),
    ("WM_NCXBUTTONUP", 172u32),
    ("WM_NEXTDLGCTL", 40u32),
    ("WM_NEXTMENU", 531u32),
    ("WM_NOTIFY", 78u32),
    ("WM_NOTIFYFORMAT", 85u32),
    ("WM_NULL", 0u32),
    ("WM_PAINT", 15u32),
    ("WM_PAINTCLIPBOARD", 777u32),
    ("WM_PAINTICON", 38u32),
    ("WM_PALETTECHANGED", 785u32),
    ("WM_PALETTEISCHANGING", 784u32),
    ("WM_PARENTNOTIFY", 528u32),
    ("WM_PASTE", 770u32),
    ("WM_PENWINFIRST", 896u32),
    ("WM_PENWINLAST", 911u32),
    ("WM_POINTERACTIVATE", 587u32),
    ("WM_POINTERCAPTURECHANGED", 588u32),
    ("WM_POINTERDEVICECHANGE", 568u32),
    ("WM_POINTERDEVICEINRANGE", 569u32),
    ("WM_POINTERDEVICEOUTOFRANGE", 570u32),
    ("WM_POINTERDOWN", 582u32),
    ("WM_POINTERENTER", 585u32),
    ("WM_POINTERHWHEEL", 591u32),
    ("WM_POINTERLEAVE", 586u32),
    ("WM_POINTERROUTEDAWAY", 594u32),
    ("WM_POINTERROUTEDRELEASED", 595u32),
    ("WM_POINTERROUTEDTO", 593u32),
    ("WM_POINTERUP", 583u32),
    ("WM_POINTERUPDATE", 581u32),
    ("WM_POINTERWHEEL", 590u32),
    ("WM_POWER", 72u32),
    ("WM_POWERBROADCAST", 536u32),
    ("WM_PRINT", 791u32),
    ("WM_PRINTCLIENT", 792u32),
    ("WM_QUERYDRAGICON", 55u32),
    ("WM_QUERYENDSESSION", 17u32),
    ("WM_QUERYNEWPALETTE", 783u32),
    ("WM_QUERYOPEN", 19u32),
    ("WM_QUERYUISTATE", 297u32),
    ("WM_QUEUESYNC", 35u32),
    ("WM_QUIT", 18u32),
    ("WM_RBUTTONDBLCLK", 518u32),
    ("WM_RBUTTONDOWN", 516u32),
    ("WM_RBUTTONUP", 517u32),
    ("WM_RENDERALLFORMATS", 774u32),
    ("WM_RENDERFORMAT", 773u32),
    ("WM_SETCURSOR", 32u32),
    ("WM_SETFOCUS", 7u32),
    ("WM_SETFONT", 48u32),
    ("WM_SETHOTKEY", 50u32),
    ("WM_SETICON", 128u32),
    ("WM_SETREDRAW", 11u32),
    ("WM_SETTEXT", 12u32),
    ("WM_SETTINGCHANGE", 26u32),
    ("WM_SHOWWINDOW", 24u32),
    ("WM_SIZE", 5u32),
    ("WM_SIZECLIPBOARD", 779u32),
    ("WM_SIZING", 532u32),
    ("WM_SPOOLERSTATUS", 42u32),
    ("WM_STYLECHANGED", 125u32),
    ("WM_STYLECHANGING", 124u32),
    ("WM_SYNCPAINT", 136u32),
    ("WM_SYSCHAR", 262u32),
    ("WM_SYSCOLORCHANGE", 21u32),
    ("WM_SYSCOMMAND", 274u32),
    ("WM_SYSDEADCHAR", 263u32),
    ("WM_SYSKEYDOWN", 260u32),
    ("WM_SYSKEYUP", 261u32),
    ("WM_TABLET_FIRST", 704u32),
    ("WM_TABLET_LAST", 735u32),
    ("WM_TCARD", 82u32),
    ("WM_THEMECHANGED", 794u32),
    ("WM_TIMECHANGE", 30u32),
    ("WM_TIMER", 275u32),
    ("WM_TOOLTIPDISMISS", 837u32),
    ("WM_TOUCH", 576u32),
    ("WM_TOUCHHITTESTING", 589u32),
    ("WM_UNDO", 772u32),
    ("WM_UNICHAR", 265u32),
    ("WM_UNINITMENUPOPUP", 293u32),
    ("WM_UPDATEUISTATE", 296u32),
    ("WM_USER", 1024u32),
    ("WM_USERCHANGED", 84u32),
    ("WM_VKEYTOITEM", 46u32),
    ("WM_VSCROLL", 277u32),
    ("WM_VSCROLLCLIPBOARD", 778u32),
    ("WM_WINDOWPOSCHANGED", 71u32),
    ("WM_WINDOWPOSCHANGING", 70u32),
    ("WM_WININICHANGE", 26u32),
    ("WM_WTSSESSION_CHANGE", 689u32),
    ("WM_XBUTTONDBLCLK", 525u32),
    ("WM_XBUTTONDOWN", 523u32),
    ("WM_XBUTTONUP", 524u32),
];
