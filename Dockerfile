from light:v0.1.0

copy earth.toml /app/
copy dist/ /app/dist/

expose 80

workdir /app

cmd ["light"]