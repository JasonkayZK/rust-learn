(function () {
    const AUTH_KEY = 'authorization'

    document.addEventListener('DOMContentLoaded', () => {
        document.getElementById(AUTH_KEY).value = localStorage.getItem(AUTH_KEY)
    })

    document.getElementById('save_auth_token_button')
        .addEventListener('click', () => {
            const auth_token = document.getElementById(AUTH_KEY).value
            localStorage.setItem(AUTH_KEY, auth_token)
        })

    const create_form = document.querySelector('#create_url_map_form')
    if (create_form) {
        create_form.addEventListener('submit', (event) => {
            event.preventDefault()
            const formData = new FormData(event.target)

            fetch('/api/url_maps', {
                method: 'POST',
                headers: {'authorization': localStorage.getItem(AUTH_KEY)},
                body: JSON.stringify(Object.fromEntries(formData)),
            }).then((response) => {
                if (response.status === 200) {
                    alert('Create Url Map successfully!')
                    window.location.href = '/web/url_maps'
                }
            })
        })
    }

    const update_form = document.getElementById('update_url_map_form')
    if (update_form) {
        update_form.addEventListener('submit', (event) => {
            event.preventDefault()
            const formData = new FormData(event.target)
            const key = formData.get('key')

            fetch(`/api/url_maps/${key}`, {
                method: 'PUT',
                headers: {'authorization': localStorage.getItem(AUTH_KEY)},
                body: JSON.stringify(Object.fromEntries(formData)),
            }).then((response) => {
                if (response.status === 200) {
                    alert(`Updated Url Map for ${key} successfully!`)
                    window.location.href = '/web/url_maps'
                }
            })
        })
    }

    const delete_url_map_links = document.querySelectorAll(".delete-url-map")
    Array.from(delete_url_map_links).forEach(link => {
        link.addEventListener("click", () => {
            const key = link.getAttribute('data')
            fetch(`/api/url_maps/${key}`, {
                method: 'DELETE',
                headers: {'authorization': localStorage.getItem(AUTH_KEY)},
            }).then((response) => {
                if (response.status === 200) {
                    alert(`Deleted Url Map for ${key} successfully!`)
                    window.location.reload()
                }
            })
        })
    })
})()
