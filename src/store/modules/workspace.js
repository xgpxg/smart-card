const state = {
    current: null
}


const mutations = {
    setWorkspace: (state, workspace) => {
        state.current = workspace
    },
}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
