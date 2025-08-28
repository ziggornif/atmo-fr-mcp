# Atmo France MCP

Serveur MCP permettant de récupérer les données de qualité de l'air en France via l'API Atmo France.

## Description

Ce projet combine les données géographiques de l'API gouvernementale française avec les indices de qualité de l'air d'Atmo France pour fournir des informations sur la qualité de l'air par ville.

## Fonctionnement

1. Récupère les codes INSEE et EPCI d'une ville via l'API geo.api.gouv.fr
2. S'authentifie auprès de l'API Atmo France
3. Récupère les données de qualité de l'air pour la zone géographique
4. Affiche l'indice de qualité de l'air avec sa description

## Configuration

Créez un fichier `.env` avec vos identifiants Atmo France :

```env
ATMO_USERNAME=votre_nom_utilisateur
ATMO_PASSWORD=votre_mot_de_passe
```

## Lancement

```bash
# Cloner le projet
git clone <url-du-repo>
cd atmo-fr-mcp

# Installer les dépendances et compiler
cargo build

# Lancer l'application
cargo run
```

## TODO

- [x] Init projet
- [x] Interfaçage API ATMO
- [x] Interfaçage API geo.api.gouv.fr
- [ ] Serveur MCP
- [ ] Documentation