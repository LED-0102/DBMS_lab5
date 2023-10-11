import axios from 'axios';

const BASE_URL = 'http://localhost:8080/auth';

const register = async (username, password) => {
    try {
        const response = await axios.post(`${BASE_URL}/register`, {
            username,
            password,
        });
        return response.data;
    } catch (error) {
        throw error.response.data;
    }
};

const login = async (username, password) => {
    try {
        const response = await axios.post(`${BASE_URL}/login`, {
            username,
            password,
        });
        return response.data.token;
    } catch (error) {
        throw error.response.data;
    }
};

export default {
    register,
    login,
};