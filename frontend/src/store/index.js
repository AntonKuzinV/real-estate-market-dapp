import { createStore } from "vuex";

export default createStore({
  state: {
    propertyToDeleteId: -1,
  },
  getters: {
    propertyToDeleteId(state) {
      return state.propertyToDeleteId;
    }
  },
  mutations: {
    setPropertyToDeleteId(state, id) {
      state.propertyToDeleteId = id;
    }
  },
  actions: {},
  modules: {},
});
