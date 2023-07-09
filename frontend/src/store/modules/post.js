import axios from "axios";

export default {
    namespaced: true,
    state: {
        posts: [],
        selected: [],
        loading: false,
        page: 1,
        totalPosts: 0,
        itemsPerPage: 10
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
        },
        setPage(state, page) {
            state.page = page
        },
        setTotalPosts(state, totalPosts) {
            state.totalPosts = totalPosts
        },
        setItemsPerPage(state, itemsPerPage) {
            state.itemsPerPage = itemsPerPage
        }
    },
    actions: {
        async fetchPosts({commit, getters}) {
            commit('setLoading', true)
            const params = {params: {page: getters.page, posts_per_page: getters.itemsPerPage}}
            await axios.get('/api/posts/', params)
                .then(response => {
                    commit('setTotalPosts', response.data.total_posts)
                    commit('setPosts', response.data.posts)
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
        async addPost({commit, dispatch}, post) {
            const form = new URLSearchParams(post)
            await axios.post('/api/posts/', form)
                .then(() => {
                    dispatch("fetchPosts")
                })
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
                .then(() => {
                    dispatch("fetchPosts")
                })
                .catch(error => {
                    console.log(error)
                })
        }
    },
    getters: {
        posts(state) {
            return state.posts
        },
        isLoading(state) {
            return state.loading
        },
        selected(state) {
            return state.selected
        },
        page(state) {
            return state.page
        },
        totalPosts(state) {
            console.log("totalPosts: " + state.totalPosts)
            return state.totalPosts
        },
        itemsPerPage(state) {
            return state.itemsPerPage
        }
    }
}