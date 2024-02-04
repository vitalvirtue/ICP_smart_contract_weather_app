export const idlFactory = ({ IDL }) => {
  const WeatherData = IDL.Record({
    'wind_speed' : IDL.Float64,
    'temperature' : IDL.Float64,
    'pressure' : IDL.Float64,
    'humidity' : IDL.Float64,
  });
  return IDL.Service({
    'get_weather_data' : IDL.Func([], [IDL.Opt(WeatherData)], []),
    'update_weather_data' : IDL.Func([WeatherData], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
