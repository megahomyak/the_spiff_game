FROM megahomyak/dum:v4 AS development
WORKDIR /app
CMD []

FROM development AS production
COPY files .
