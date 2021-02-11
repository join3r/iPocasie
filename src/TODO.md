1. Lokaciu
    - cez parameter
    - cez ipgeolocate

    Zadaj lokaciu (ipgeolocate lokacia):
2. Poslať API request https://openweathermap.org/api/geocoding-api
    osiris
    http://api.openweathermap.org/geo/1.0/direct?q={city name},{state code},{country code}&limit=1&appid=3a4f66679a08357803b5c2134b4ad9cb

    example : http://api.openweathermap.org/geo/1.0/direct?q=London&limit=5&appid=303cad08c90cc40c53a5d1afb6fba1c6


3. Poslat API request https://openweathermap.org/api/one-call-api
4. Parsovat response
5. Speak