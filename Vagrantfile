# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.define "fbsd_13_1" do |fbsd_13_1|
    fbsd_13_1.vm.box = "freebsd/FreeBSD-13.1-RELEASE"
  end

  config.vm.define "fbsd_12_4" do |fbsd_12_4|
    fbsd_12_4.vm.box = "freebsd/FreeBSD-12.4-STABLE"
  end

  config.vm.synced_folder ".", "/vagrant", type: "rsync",
    rsync__auto: true

  config.vm.provision "shell", inline: <<-SHELL
    pkg bootstrap
    pkg install -y rust
  SHELL
end
