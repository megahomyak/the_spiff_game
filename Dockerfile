FROM megahomyak/dum:v5 AS development
WORKDIR /app
CMD []

FROM development AS production
COPY files .
