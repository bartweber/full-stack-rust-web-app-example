import axios from "axios";

export default {
    namespaced: true,
    state: {
        posts: [],
        selected: [],
        loading: false,
    },
    mutations: {
        setLoading(state, loading) {
            state.loading = loading
        },
        setPosts(state, posts) {
            state.posts = posts
        },
        setSelected(state, selected) {
            state.selected = selected
        }
    },
    actions: {
        async fetchPosts({commit}) {
            commit('setLoading', true)
            await axios.get('/api/posts/')
                .then(response => {
                    commit('setPosts', response.data)
                })
                .catch(error => {
                    console.log(error)
                })
                .finally(() => {
                    commit('setLoading', false)
                })
        },
        async fetchPost({commit}, id) {
            try {
                const response = await axios.get(`/api/posts/${id}`)
                return response.data
            } catch (error) {
                console.log(error)
            }
        },
        async addPost({commit}, post) {
            const form = new URLSearchParams(post)
            await axios.post('/api/posts/', form)
                .catch(error => {
                    console.log(error)
                })
        },
        async deletePost({dispatch, state, commit}, id) {
            axios.delete(`/api/posts/${id}`)
                .then(() => {
                    // fix the array of the state variable 'selected'
                    const selected = state.selected.filter(item => item !== id)
                    commit('setSelected', selected)
                })
                .then(() => {
                    dispatch("fetchPosts")
                })
                .catch(error => {
                    console.log(error)
                })
        },
        async deletePosts({dispatch, commit}, ids) {
            const patch = ids.map(id => {
                return {"op": "remove", "path": `/api/posts/${id}`, "value": ""}
            })
            await axios.patch(`/api/posts/`, patch)
                .then(() => {
                    dispatch("fetchPosts")
                })
                .then(() => {
                    commit('setSelected', [])
                })
                .catch(error => {
                    console.log(error)
                })
        },
        async updatePost({dispatch}, post) {
            const form = new URLSearchParams(post)
            await axios.put(`/api/posts/${post.id}`, form)
                .catch(error => {
                    console.log(error)
                })
        }
    },
    getters: {
        allPosts(state) {
            return state.posts
        },
        postCount(state) {
            return state.posts.length
        },
        isLoading(state) {
            return state.loading
        },
        selected(state) {
            return state.selected
        }
    }
}