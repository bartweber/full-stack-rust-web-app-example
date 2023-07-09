import {createStore} from 'vuex'
import counter from './modules/counter'
import post from './modules/post'

export default createStore({
    modules: {
        counter,
        post,
    },
})