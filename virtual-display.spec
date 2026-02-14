# Create an option to build locally without fetchting own repo
# for sourcing and patching
%bcond local 0

# Source repo
%global author pvermeer
%global source virtual-display
%global sourcerepo https://github.com/PVermeer/virtual-display
%global tag v0.0.2

Name: virtual-display
Version: 0.0.2
Release: 0%{?dist}
License: GPL-3.0 license
Summary: Enable a virtual kernel display
Url: %{sourcerepo}

BuildRequires: systemd-rpm-macros
BuildRequires: git
BuildRequires: rustup
BuildRequires: gcc
BuildRequires: make
BuildRequires: pkgconf-pkg-config
BuildRequires: glibc-devel

%description
A daemon and cli to temporary enable/disable a virtual display via the kernel debug sys paths.

%define workdir %{_builddir}/%{name}
%define sourcedir %{workdir}/%{source}
%define service virtual-display-daemon.service
%define service_preset 50-virtual-display-daemon.preset

%prep
# To apply working changes handle sources / patches locally
# COPR should clone the commited changes
%if %{with local}
  # Get sources - local build
  mkdir -p %{sourcedir}
  cp -r %{_topdir}/SOURCES/* %{sourcedir}
%else
  # Get sources - COPR build
  git clone %{sourcerepo} %{sourcedir}
  cd %{sourcedir}
  git reset --hard %{tag}
  cd %{workdir}
%endif

# Do src stuff
cd %{sourcedir}
rm -rf .git
cd %{workdir}

%build
cd %{sourcedir}
rustup-init -y
source "$HOME/.cargo/env"
cargo build --release

%check

%install
mkdir -p %{buildroot}%{_unitdir}
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_presetdir}

install %{sourcedir}/assets/%{service} %{buildroot}%{_unitdir}
install %{sourcedir}/assets/%{service_preset} %{buildroot}%{_presetdir}
install %{sourcedir}/target/release/virtual-display-daemon %{buildroot}%{_bindir}
install %{sourcedir}/target/release/virtual-display %{buildroot}%{_bindir}

%post
%systemd_post %{service}
systemctl start %{service}

%preun
%systemd_preun %{service}

%postun
%systemd_postun_with_restart %{service}

%files
/%{_unitdir}/%{service}
/%{_presetdir}/%{service_preset}
/%{_bindir}/virtual-display-daemon
/%{_bindir}/virtual-display
