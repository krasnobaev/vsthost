import { applyMiddleware, createStore } from 'redux';
import { createEpicMiddleware } from 'redux-observable';
import logger from 'redux-logger';

import { rootEpic, rootReducer } from './duck1';

let state = {};

const epicMiddleware = createEpicMiddleware(rootEpic);

export const configureStore = (initialState = state) => {
  return createStore(
    rootReducer,
    initialState,
    applyMiddleware(
      epicMiddleware,
      logger
    )
  );
};
