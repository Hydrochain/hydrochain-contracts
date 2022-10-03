/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.17.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export interface InstantiateMsg {}
export type ExecuteMsg = {
  produce: {
    color_spectrum: ColorSpectrum;
    price: Coin;
    volume: number;
  };
} | {
  update_price: {
    container_id: number;
    new_price: Coin;
  };
} | {
  buy: {
    container_id: number;
    coordinates: Coordinates;
    destination: string;
  };
} | {
  close_shipment: {
    container_id: number;
  };
} | {
  remove_container: {
    container_id: number;
  };
};
export type ColorSpectrum = "GREEN" | "YELLOW" | "BLUE" | "WHITE" | "BLACK";
export type Uint128 = string;
export type Decimal = string;
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export interface Coordinates {
  latitude: Decimal;
  longtitude: Decimal;
  [k: string]: unknown;
}
export type QueryMsg = {
  container: {
    container_id: number;
  };
} | {
  containers: {};
} | {
  containers_by_owner: {
    owner: string;
  };
};
export type Addr = string;
export type Status = ("Created" | "Delivered") | {
  Shipped: ShipmentDetails;
};
export interface ContainerResponse {
  color_spectrum: ColorSpectrum;
  container_id: number;
  owner: Addr;
  price: Coin;
  status: Status;
  volume: number;
}
export interface ShipmentDetails {
  buyer: Addr;
  coordinates: Coordinates;
  destination: string;
  [k: string]: unknown;
}
export interface ContainersResponse {
  containers: ContainerResponse[];
}