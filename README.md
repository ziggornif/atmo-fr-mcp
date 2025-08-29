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

- Créer un compte sur le site Atmo France : https://admindata.atmo-france.org/inscription-api
- Initialiser le mot de passe via le mail reçu
- Créez un fichier `.env` avec vos identifiants Atmo France créés :

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

Le serveur sera disponible sur `http://127.0.0.1:8000/mcp`.

## Serveur MCP

Ce projet implémente un serveur MCP (Model Context Protocol) qui expose un outil pour récupérer les données de qualité de l'air.

### Configuration MCP VSCode

Pour utiliser ce serveur MCP avec VSCode, ajoutez la configuration suivante dans vos paramètres MCP :


```json
{
	"servers": {
		"atmo-fr-mcp": {
			"url": "http://localhost:8000/mcp",
			"type": "http"
		}
	}
}
```

### Outils disponibles

- **get_air_quality** : Récupère la qualité de l'air pour une ville donnée
  - Paramètres :
    - `ville` : Nom de la ville
    - `code_postal` : Code postal de la ville
  - Retourne : Informations détaillées sur la qualité de l'air incluant l'indice global et les polluants spécifiques

### Exemple

> Peux tu me donner la qualité de l'air à Vannes ?

> La qualité de l'air à Vannes (56000) aujourd'hui est "Moyen" (indice 2).
> 
> Détail des polluants :
> 
> - NO₂ (dioxyde d'azote) : 1
> - O₃ (ozone) : 2
> - PM10 (particules fines) : 1
> - PM2.5 (particules très fines) : 1
> - SO₂ (dioxyde de soufre) : 1
>
> Source : Air Breizh, zone CA Golfe du Morbihan - Vannes Agglomération.