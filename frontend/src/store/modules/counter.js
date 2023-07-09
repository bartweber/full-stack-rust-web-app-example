import axios from "axios";

export default {
    namespaced: true,
    state: {
        counter: 0,
        colorCode: 'blue'
    },
    mutations: {
        increment(state, randomNumber) {
            state.counter += randomNumber
        },
        decrement(state, randomNumber) {
            state.counter -= randomNumber
        },
        setColorCode(state, colorCode) {
            state.colorCode = colorCode
        }
    },
    actions: {
        increment({commit}) {
            axios.get('https://www.random.org/integers/?num=1&min=1&max=6&col=1&base=10&format=plain&rnd=new')
                .then(response => {
                    commit('increment', response.data)
                })
        },
        decrement({commit}) {
            axios.get('https://www.random.org/integers/?num=1&min=1&max=6&col=1&base=10&format=plain&rnd=new')
                .then(response => {
                    commit('decrement', response.data)
                })
        },
        setColorCode({commit}, colorCode) {
            commit('setColorCode', colorCode)
        },
    },
    getters: {
        counterSquared(state) {
            return state.counter * state.counter
        }
    },
}