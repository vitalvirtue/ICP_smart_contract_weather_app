import { icp_smart_contract_weather_app_backend } from "../../declarations/icp_smart_contract_weather_app_backend";

document.getElementById('getWeatherButton').addEventListener('click', async function() {
    try {
        const weatherData = await icp_smart_contract_weather_app_backend.get_weather_data();
        document.getElementById('weatherData').innerText = JSON.stringify(weatherData);
    } catch (error) {
        console.error('Error:', error);
    }
});
