FROM megahomyak/dum:v3 AS development
WORKDIR /app
CMD []

FROM development AS production
COPY files .
