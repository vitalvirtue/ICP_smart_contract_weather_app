import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface State { 'weather_data' : [] | [WeatherData] }
export interface WeatherData {
  'wind_speed' : number,
  'temperature' : number,
  'pressure' : number,
  'humidity' : number,
}
export interface _SERVICE {
  'get_weather_data' : ActorMethod<[], [] | [WeatherData]>,
  'update_weather_data' : ActorMethod<[WeatherData], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
