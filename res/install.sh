#!/bin/sh

cp ./ant-rs /usr/bin/ant-rs
chmod 755 /usr/bin/ant-rs

mkdir -p /usr/share/doc/ant-rs/
cp ./README /usr/share/doc/ant-rs/README
chmod 644 /usr/share/doc/ant-rs/README

mkdir -p /usr/share/man/man8/
cp ./ant-rs.8 /usr/share/man/man8/ant-rs.8
chmod 644 /usr/share/man/man8/ant-rs.8
gzip /usr/share/man/man8/ant-rs.8
