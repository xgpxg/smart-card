import {createStore} from 'vuex'
import getters from './getters'
import user from './modules/user'
import workspace from "./modules/workspace";

const store = createStore({
    modules: {
        user,
        workspace
    },
    getters
})


export {
    store
}
