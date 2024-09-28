document.addEventListener('DOMContentLoaded', async function() {
  try {
    await fetch("http://localhost:2388")
    window.location.href = "found.html";
  } catch (e) {
    window.location.href = "not_found.html";
  }
});
