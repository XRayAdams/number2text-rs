%define _name number2text
%define _version 2.2.1
%define _release 23
%define debug_package %{nil}

Name: %{_name}
Version: %{_version}
Release: %{_release}
Summary: Number to Text Converter
License: MIT
Group: Applications/Utilities
URL: https://github.com/XRayAdams/number2text-rs
BugURL: https://github.com/XRayAdams/number2text-rs/issues
Vendor: Konstantin Adamov

Source0: %{_name}-%{_version}.tar.gz
Source1: app.rayadams.number2text.desktop
Source2: app.rayadams.number2text.png
Source3: app.rayadams.number2text.metainfo.xml

Requires: gtk4, libadwaita, libstdc++

%description
Number 2 Text is a free, open-source application to convert numbers to text.

Supported Languages
English
Spanish
German
French
Russian


%prep
%setup -q -n release

%build
# This section is intentionally left blank as we are packaging a pre-compiled Flutter application.

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps
mkdir -p %{buildroot}/opt/%{_name}
mkdir -p %{buildroot}%{_datadir}/metainfo

# Copy the application files
cp -r ./* %{buildroot}/opt/%{_name}/

# Create a symlink in /usr/bin
ln -s /opt/%{_name}/%{_name} %{buildroot}/usr/bin/%{_name}

# Copy the desktop file
install -m 644 %{SOURCE1} %{buildroot}/usr/share/applications/%{_name}.desktop

# Copy the application icon
install -m 644 %{SOURCE2} %{buildroot}/usr/share/icons/hicolor/256x256/apps/%{_name}.png

# Copy meta info
install -m 644 %{SOURCE3} %{buildroot}%{_datadir}/metainfo/%{name}.metainfo.xml
%files
/usr/bin/%{_name}
/opt/%{_name}
/usr/share/applications/%{_name}.desktop
/usr/share/icons/hicolor/256x256/apps/%{_name}.png
%{_datadir}/metainfo/%{name}.metainfo.xml

%changelog
*loghere
- Initial RPM release
