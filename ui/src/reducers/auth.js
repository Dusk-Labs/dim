import {
  AUTH_LOGIN_START,
  AUTH_LOGIN_OK,
  AUTH_LOGIN_ERR,
  AUTH_UPDATE_TOKEN,
  AUTH_LOGOUT,
  AUTH_REGISTER_ERR,
  AUTH_REGISTER_OK,
  AUTH_REGISTER_START,
  AUTH_CHECK_ADMIN_OK,
  CREATE_NEW_INVITE_START,
  CREATE_NEW_INVITE_OK,
  CREATE_NEW_INVITE_ERR,
  FETCH_INVITES_START,
  FETCH_INVITES_OK,
  FETCH_INVITES_ERR
} from "../actions/types.js";

const login = {
  logging_in: false,
  logged_in: false,
  error: null
};

const register = {
  registering: false,
  registered: false,
  error: null
};

const createNewInvite = {
  code: "",
  creating: false,
  created: false,
  error: null
};

const invites = {
  items: [],
  fetching: false,
  fetched: false,
  error: null
};

const initialState = {
  token: null,
  admin_exists: false,
  login,
  register,
  createNewInvite,
  invites
};

export default function authReducer(state = initialState, action) {
  switch(action.type) {
    case AUTH_LOGIN_START:
      return {
        ...state,
        token: null,
        login: {
          logging_in: true,
          logged_in: false,
          error: null
        }
      };
    case AUTH_LOGIN_ERR:
      return {
        ...state,
        login: {
          ...state.login,
          logging_in: false,
          logged_in: false,
          error: action.payload
        }
      };
    case AUTH_LOGIN_OK:
      return {
        ...state,
        token: action.payload.token,
        login: {
          ...state.login,
          logging_in: false,
          logged_in: true
        }
      };
    case AUTH_UPDATE_TOKEN:
      return {
        ...state,
        token: action.payload,
        login: {
          ...state.login,
          logging_in: false,
          logged_in: true,
          error: null
        }
      };
    case AUTH_LOGOUT:
      return initialState;
    case AUTH_REGISTER_START:
      return {
        ...state,
        register: {
          ...state.register,
          registering: true,
          registered: false,
          error: null
        }
      };
    case AUTH_REGISTER_OK:
      return initialState;
    case AUTH_REGISTER_ERR:
      return {
        ...state,
        register: {
          registering: false,
          registered: false,
          error: action.payload
        }
      };
    case AUTH_CHECK_ADMIN_OK:
      return {
        ...state,
        admin_exists: action.payload.exists
      };
    case CREATE_NEW_INVITE_START:
      return {
        ...state,
        createNewInvite: {
          code: "",
          creating: true,
          created: false,
          error: null
        }
      };
    case CREATE_NEW_INVITE_ERR:
      return {
        ...state,
        createNewInvite: {
          ...state.createNewInvite,
          creating: false,
          created: false,
          error: action.payload
        }
      };
    case CREATE_NEW_INVITE_OK:
      return {
        ...state,
        createNewInvite: {
          ...state.createNewInvite,
          creating: false,
          created: true,
          code: action.payload.token
        }
      };
    case FETCH_INVITES_START:
      return {
        ...state,
        invites: {
          ...state.invites,
          fetching: true,
          fetched: false,
          error: null
        },
        createNewInvite: {
          code: "",
          creating: false,
          created: false,
          error: null
        }
      };
    case FETCH_INVITES_ERR:
      return {
        ...state,
        invites: {
          ...state.invites,
          fetching: false,
          fetched: false,
          error: action.payload
        }
      };
    case FETCH_INVITES_OK:
      return {
        ...state,
        invites: {
          ...state.invites,
          items: action.payload,
          fetching: false,
          fetched: true
        }
      };
    default:
      return state;
  }
}
