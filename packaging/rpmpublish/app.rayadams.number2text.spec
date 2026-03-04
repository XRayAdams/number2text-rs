%bcond_without check

%global upstreamver 2.2.2+32

Name:           number2text
Version:        2.2.2
Release:        32%{?dist}
Summary:        Number to Text Converter
License:        MIT
URL:            https://github.com/XRayAdams/number2text-rs
Source0:        %{url}/archive/refs/tags/v%{upstreamver}/%{name}-rs-%{upstreamver}.tar.gz
Source1:        %{name}-%{upstreamver}-vendor.tar.xz

BuildRequires:  rust-packaging
BuildRequires:  cargo-rpm-macros
BuildRequires:  gtk4-devel
BuildRequires:  libadwaita-devel

%description
Number 2 Text is a free, open-source application to convert numbers
to text. It supports the following languages: English, Spanish, German,
French, Italian, Russian.

%prep
%autosetup -n %{name}-rs-%(echo %{upstreamver} | tr '+' '-')
tar xf %{SOURCE1}
%cargo_prep -v vendor

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
# Binary
install -D -p -m 0755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Desktop file
install -D -p -m 0644 packaging/gui/app.rayadams.%{name}.desktop \
    %{buildroot}%{_datadir}/applications/app.rayadams.%{name}.desktop

# Icon
install -D -p -m 0644 packaging/gui/app.rayadams.%{name}.png \
    %{buildroot}%{_datadir}/icons/hicolor/256x256/apps/app.rayadams.%{name}.png

# AppStream metainfo
install -D -p -m 0644 packaging/app.rayadams.%{name}.metainfo.xml \
    %{buildroot}%{_metainfodir}/app.rayadams.%{name}.metainfo.xml

# Man page
install -D -p -m 0644 packaging/%{name}.1 \
    %{buildroot}%{_mandir}/man1/%{name}.1

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/applications/app.rayadams.%{name}.desktop
%{_datadir}/icons/hicolor/256x256/apps/app.rayadams.%{name}.png
%{_metainfodir}/app.rayadams.%{name}.metainfo.xml
%{_mandir}/man1/%{name}.1*

%changelog
* Wed Mar 04 2026 Konstantin Adamov <xrayadamo@gmail.com> - 2.2.2-32
- Initial Fedora packaging
