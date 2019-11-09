import Vue from 'vue';
import Vuex, { StoreOptions } from 'vuex';
import { RootState } from '@/types';
import Cell from '@/engine/Cell';
import {
  SET_GAME_STATE,
  SET_SIMULATION_STATE,
  SET_ACTIVE_CELL,
  RESET_ACTIVE_CELL,
  SET_HOVERED_CELL
} from './mutation-types';
import optionsModule from './optionsModule';

const initialCell = Cell.createDummy();
Vue.use(Vuex);

const store: StoreOptions<RootState> = {
  state: {
    activeCell: initialCell,
    cellSelected: false,
    hoveredCell: initialCell,
    gameState: 'Initial',
    simulationState: false
  },
  mutations: {
    // set active level
    [SET_GAME_STATE](state, gameState) {
      state.gameState = gameState;
    },
    // set active level
    [SET_SIMULATION_STATE](state, simulationState) {
      state.simulationState = simulationState;
    },
    // set active cell
    [SET_ACTIVE_CELL](state, cell) {
      state.activeCell = cell;
      state.cellSelected = true;
    },
    // reset active cell
    [RESET_ACTIVE_CELL](state) {
      state.activeCell = initialCell;
      state.cellSelected = false;
    },
    // hovered cell functional
    [SET_HOVERED_CELL](state, cell) {
      state.hoveredCell = cell;
    }
  },
  modules: {
    optionsModule
  }
};

export default new Vuex.Store<RootState>(store);
