1. Lokaciu
    - cez parameter
    - cez ipgeolocate

    Zadaj lokaciu (ipgeolocate lokacia):

3. Poslat API request https://openweathermap.org/api/one-call-api
    {{baseUrl}}/weather?q=<CITY>&units=metric&lang=<LANGUAGE>&appid=<API KEY>
    {{baseUrl}}/forecast?q=<CITY>&units=metric&lang=<LANGUAGE>&appid=<API KEY>

4. Parsovat response
5. Speak

Notes:
Json -> Serde struct = https://transform.tools/json-to-rust-serde