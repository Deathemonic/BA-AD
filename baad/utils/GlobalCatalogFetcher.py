import re

import requests
from requests_cache import CachedSession

PLAYSTORE_URL = "https://play.google.com/store/apps/details?id=com.nexon.bluearchive&hl=in&gl=US"
API_URL = "https://api-pub.nexon.com/patch/v1.1/version-check"


def get_game_version() -> str:
    version_pattern = re.compile(r"\d{1}\.\d{2}\.\d{6}")
    response = requests.get(PLAYSTORE_URL)
    version_match = version_pattern.search(response.text)
    if not version_match:
        raise ValueError("Could not find game version in Play Store page")
    return version_match.group()


def catalog_url() -> str:
    version = get_game_version()
    build_number = version.split('.')[-1]

    with CachedSession('nexonapi', use_temp=True) as session:
        response = session.post(
            API_URL,
            json={
                "market_game_id": "com.nexon.bluearchive",
                "market_code": "playstore",
                "curr_build_version": version,
                "curr_build_number": build_number
            }
        )
        data = response.json()
        return data['patch']['resource_path']