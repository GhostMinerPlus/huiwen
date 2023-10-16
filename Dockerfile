from light:v0.1.0

copy earth.toml /
copy dist/ /dist/

expose 80

cmd ["/bin/light"]