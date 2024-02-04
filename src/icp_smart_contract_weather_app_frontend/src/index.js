import { icp_smart_contract_weather_app_backend } from "../../declarations/icp_smart_contract_weather_app_backend";

async function getWeatherData() {
    const button = document.querySelector("button");
    button.setAttribute("disabled", true);

    // Interact with Rust actor, calling the get_weather_data method
    const data = await icp_smart_contract_weather_app_backend.update_weather_data();

    button.removeAttribute("disabled");

    // Alınan veriler HTML elementlerine eklenir.
    const weatherDataDiv = document.getElementById('weatherData');
    weatherDataDiv.innerHTML = `
        <p>Temperature: ${data.temperature}</p>
        <p>Pressure: ${data.pressure}</p>
        <p>Humidity: ${data.humidity}</p>
        <p>Wind Speed: ${data.wind_speed}</p>
    `;
}

