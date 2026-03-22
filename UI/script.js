async function findInPi() {
    const type = document.getElementById('searchType').value;
    const query = document.getElementById('searchQuery').value;
    const res = document.getElementById('result');
    const status = document.getElementById('status');

    if (!query) return;

    status.innerText = "Searching Pi...";
    res.innerText = "";

    try {
        // PRODUCTION FIX: Use relative paths for Render (HTTPS compatible)
        const response = await fetch(`/search/${type}/${encodeURIComponent(query)}`);
        
        if (!response.ok) {
            const errorData = await response.json();
            res.innerText = errorData.message || "Sequence not found.";
            status.innerText = "Not found.";
            return;
        }

        const data = await response.json();
        status.innerText = "Success!";
        res.innerText = data.message;
    } catch (err) {
        console.error(err);
        res.innerText = "Error connecting to the Pi engine.";
        status.innerText = "Connection failed.";
    }
}
