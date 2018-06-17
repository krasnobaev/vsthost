import { combineReducers } from 'redux';
import Rx from 'rxjs/Rx';
import { BehaviorSubject } from 'rxjs/BehaviorSubject';
import { combineEpics } from 'redux-observable';

/* actions */

const INITIALIZE_MAIN_MODULE = 'INITIALIZE_MAIN_MODULE';

/* action creators */

export const initializeBattlefield = (oInitialState) => {
  return { type: INITIALIZE_MAIN_MODULE, oInitialState };
};

/* epics */

const initializeEpic = (action$, store) => action$
  .ofType(INITIALIZE_MAIN_MODULE)
  .delay(1000);

const rootEpic = combineEpics(
  initializeEpic
);

/* reducers */

const moduleMainState = (state = {
}, {
  type,
  oInitialState,
} = action) => {

  switch (type) {
    case INITIALIZE_MAIN_MODULE:
      return Object.assign({}, state, oInitialState);

    default:
      return state;
  }
};

export const rootReducer = combineReducers({
  moduleMainState
});
