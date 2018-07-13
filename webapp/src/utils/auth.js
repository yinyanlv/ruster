export default {
    user: {
		authenticated: false,
		role: null
    }, 
    checkAuth() {
		var jwt = sessionStorage.getItem('token');
		var role = sessionStorage.getItem('signin_user');
		if (jwt) {
			this.user.authenticated = true;
			this.user.role = role;
		} else {
			this.user.authenticated = false;
		}
    },

    getAuthHeader () {
        return {
            headers: {
                'Authorization': 'Bearer ' + sessionStorage.getItem('token')
            }
        }
    }
}
