export default {
    player: {
        allowedDesync: 2
    },
    log: {
        '': 'none',
        auth: 'warn'
    },
    auth: {
        localStorageKey: 'apelleUser'
    },
    notifications: {
        timeout: 3000
    },
    search: {
        page_size: 5
    }
} as const;
