# Roadmap
## 1. Features
### 1.1. Window
- [x] Main window creation
- [ ] Sub window creation
- [ ] Icon assignment
- [ ] Changing window title
- [ ] Window Menu

### 1.2. Components
#### 1.2.1. Interactive
- [x] Button
- [ ] Calendar
- [ ] Check Box
- [ ] Text Field (single line)
- [ ] Text Field (multiline)

#### 1.2.2. Interactive Containers
- [ ] Rich Text Document ([NSDocument](https://developer.apple.com/documentation/appkit/documents_data_and_pasteboard/developing_a_document-based_app?language=objc), [Rich Edit](https://learn.microsoft.com/en-us/windows/win32/controls/rich-edit-controls))
- [ ] Select List

#### 1.2.3. Static
- [x] Label
- [ ] List
- [ ] Image
- [ ] Tree

#### 1.2.4. Helpers
- [ ] Grid
- [ ] Panel
- [ ] Scroll Box
- [ ] Stack (Horizontal, Vertical)
- [ ] Table
- [ ] Tab View

#### 1.2.5. Special
- [ ] Canvas (2D Graphics)
- [ ] Canvas (Metal/DirectX)
- [ ] Web View

### 1.3. Customization
- [ ] Alignment
- [x] Setting foreground color of text components
- [ ] Setting background color
- [ ] Setting font/typeface
- [x] Tool Tips

#### 1.3.1. Theming
- [ ] Honor System Theme by Default
- [ ] Overriding Light/Dark Mode

### 1.5. Miscellaneous
- [x] Dialog Box
- [ ] File Open Dialog
- [ ] File Save Dialog
- [ ] I18n PO/gettext translation

## 2. Platform Support
- [x] macOS/AppKit
- [x] Windows/Win32
- [ ] Windows/WinUI
- [ ] Linux/GTK

## 3. Tooling
- [ ] Code Signing
- [ ] Packaging ([MSIX](https://learn.microsoft.com/en-us/windows/msix/)/[APPX](https://learn.microsoft.com/en-us/windows/win32/appxpkg/appx-portal)/[MSI](https://learn.microsoft.com/en-us/windows/win32/msi/windows-installer-portal), [DMG](https://en.wikipedia.org/wiki/Apple_Disk_Image)/[APP](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/Introduction/Introduction.html))
- [ ] UI Automation (Testing)
