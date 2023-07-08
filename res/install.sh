#!/bin/sh

cp bin/ant-rs /usr/bin/
chmod 755 /usr/bin/ant-rs

mkdir -p /usr/share/doc/ant-rs/
cp docs/README /usr/share/doc/ant-rs/
chmod 644 /usr/share/doc/ant-rs/README

mkdir -p /usr/share/man/man8/
cp man/ant-rs.8 /usr/share/man/man8/
gzip /usr/share/man/man8/ant-rs.8
chmod 644 /usr/share/man/man8/ant-rs.8.gz

mkdir -p /usr/lib/systemd/system/
cp systemd/ant-rs-daemon.service /usr/lib/systemd/system/
cp -n systemd/ant.yaml           /etc/ssh
chmod 644 /etc/ssh/ant.yaml
chmod 644 /usr/lib/systemd/system/ant-rs-daemon.service
