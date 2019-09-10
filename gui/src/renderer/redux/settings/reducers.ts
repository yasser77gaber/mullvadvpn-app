import log from 'electron-log';
import {
  BridgeState,
  KeygenEvent,
  LiftedConstraint,
  RelayLocation,
  RelayProtocol,
  TunnelProtocol,
} from '../../../shared/daemon-rpc-types';
import { IGuiSettingsState } from '../../../shared/gui-settings-state';
import { ReduxAction } from '../store';

export type RelaySettingsRedux =
  | {
      normal: {
        tunnelProtocol: LiftedConstraint<TunnelProtocol>;
        location: LiftedConstraint<RelayLocation>;
        openvpn: {
          port: LiftedConstraint<number>;
          protocol: LiftedConstraint<RelayProtocol>;
        };
        wireguard: {
          port: LiftedConstraint<number>;
        };
      };
    }
  | {
      customTunnelEndpoint: {
        host: string;
        port: number;
        protocol: RelayProtocol;
      };
    };

export interface IRelayLocationRelayRedux {
  hostname: string;
  ipv4AddrIn: string;
  includeInCountry: boolean;
  weight: number;
}

export interface IRelayLocationCityRedux {
  name: string;
  code: string;
  latitude: number;
  longitude: number;
  hasActiveRelays: boolean;
  relays: IRelayLocationRelayRedux[];
}

export interface IRelayLocationRedux {
  name: string;
  code: string;
  hasActiveRelays: boolean;
  cities: IRelayLocationCityRedux[];
}

export interface IWgKey {
  publicKey: string;
  created: string;
  valid?: boolean;
  replacementFailure?: KeygenEvent;
  verificationFailed?: boolean;
}

interface IWgKeySet {
  type: 'key-set';
  key: IWgKey;
}

interface IWgKeyNotSet {
  type: 'key-not-set';
}

interface IWgTooManyKeys {
  type: 'too-many-keys';
}

interface IWgKeyGenerationFailure {
  type: 'generation-failure';
}

interface IWgKeyBeingGenerated {
  type: 'being-generated';
}

interface IWgKeyBeingReplaced {
  type: 'being-replaced';
  oldKey: IWgKey;
}

interface IWgKeyBeingVerified {
  type: 'being-verified';
  key: IWgKey;
}

export type WgKeyState =
  | IWgKeySet
  | IWgKeyNotSet
  | IWgKeyGenerationFailure
  | IWgTooManyKeys
  | IWgKeyBeingVerified
  | IWgKeyBeingReplaced
  | IWgKeyBeingGenerated;

export interface ISettingsReduxState {
  autoStart: boolean;
  guiSettings: IGuiSettingsState;
  relaySettings: RelaySettingsRedux;
  relayLocations: IRelayLocationRedux[];
  allowLan: boolean;
  enableIpv6: boolean;
  bridgeState: BridgeState;
  blockWhenDisconnected: boolean;
  openVpn: {
    mssfix?: number;
  };
  wireguardKeyState: WgKeyState;
}

const initialState: ISettingsReduxState = {
  autoStart: false,
  guiSettings: {
    enableSystemNotifications: true,
    autoConnect: true,
    monochromaticIcon: false,
    startMinimized: false,
  },
  relaySettings: {
    normal: {
      location: 'any',
      tunnelProtocol: 'any',
      wireguard: { port: 'any' },
      openvpn: {
        port: 'any',
        protocol: 'any',
      },
    },
  },
  relayLocations: [],
  allowLan: false,
  enableIpv6: true,
  bridgeState: 'auto',
  blockWhenDisconnected: false,
  openVpn: {},
  wireguardKeyState: {
    type: 'key-not-set',
  },
};

export default function(
  state: ISettingsReduxState = initialState,
  action: ReduxAction,
): ISettingsReduxState {
  switch (action.type) {
    case 'UPDATE_GUI_SETTINGS':
      return {
        ...state,
        guiSettings: action.guiSettings,
      };

    case 'UPDATE_RELAY':
      return {
        ...state,
        relaySettings: action.relay,
      };

    case 'UPDATE_RELAY_LOCATIONS':
      return {
        ...state,
        relayLocations: action.relayLocations,
      };

    case 'UPDATE_ALLOW_LAN':
      return {
        ...state,
        allowLan: action.allowLan,
      };

    case 'UPDATE_ENABLE_IPV6':
      return {
        ...state,
        enableIpv6: action.enableIpv6,
      };

    case 'UPDATE_BLOCK_WHEN_DISCONNECTED':
      return {
        ...state,
        blockWhenDisconnected: action.blockWhenDisconnected,
      };

    case 'UPDATE_OPENVPN_MSSFIX':
      return {
        ...state,
        openVpn: {
          ...state.openVpn,
          mssfix: action.mssfix,
        },
      };

    case 'UPDATE_AUTO_START':
      return {
        ...state,
        autoStart: action.autoStart,
      };

    case 'UPDATE_BRIDGE_STATE':
      return {
        ...state,
        bridgeState: action.bridgeState,
      };

    case 'SET_WIREGUARD_KEY':
      return {
        ...state,
        wireguardKeyState: setWireguardKey(action.key),
      };
    case 'WIREGUARD_KEYGEN_EVENT':
      return {
        ...state,
        wireguardKeyState: setWireguardKeygenEvent(state, action.event),
      };
    case 'WIREGUARD_KEY_VERIFICATION_COMPLETE':
      return {
        ...state,
        wireguardKeyState: applyKeyVerification(state.wireguardKeyState, action.verified),
      };
    case 'VERIFY_WIREGUARD_KEY':
      return {
        ...state,
        wireguardKeyState: { type: 'being-verified', key: action.key },
      };

    case 'GENERATE_WIREGUARD_KEY':
      return {
        ...state,
        wireguardKeyState: { type: 'being-generated' },
      };

    case 'REPLACE_WIREGUARD_KEY':
      return {
        ...state,
        wireguardKeyState: { type: 'being-replaced', oldKey: action.oldKey },
      };

    default:
      return state;
  }
}

function setWireguardKey(key?: IWgKey): WgKeyState {
  if (key) {
    return {
      type: 'key-set',
      key,
    };
  } else {
    return {
      type: 'key-not-set',
    };
  }
}

function setWireguardKeygenEvent(state: ISettingsReduxState, keygenEvent: KeygenEvent): WgKeyState {
  const oldKeyState = state.wireguardKeyState;
  if (oldKeyState.type === 'being-replaced') {
    switch (keygenEvent) {
      case 'too_many_keys':
      case 'generation_failure':
        return {
          type: 'key-set',
          key: {
            ...oldKeyState.oldKey,
            replacementFailure: keygenEvent,
          },
        };
      default:
        break;
    }
  }
  switch (keygenEvent) {
    case 'too_many_keys':
      return { type: 'too-many-keys' };
    case 'generation_failure':
      return { type: 'generation-failure' };
    default:
      return {
        type: 'key-set',
        key: {
          publicKey: keygenEvent.newKey.key,
          created: keygenEvent.newKey.created,
          valid: undefined,
        },
      };
  }
}

function applyKeyVerification(state: WgKeyState, verified?: boolean): WgKeyState {
  const verificationFailed = verified === undefined ? true : undefined;
  switch (state.type) {
    case 'being-verified':
      return {
        type: 'key-set',
        key: {
          ...state.key,
          valid: verified,
          verificationFailed,
        },
      };
    // drop the verification event if the key wasn't being verified.
    default:
      log.error(`Received key verification event when key wasn't being verified`);
      return state;
  }
}
