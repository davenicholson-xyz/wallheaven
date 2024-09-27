document.addEventListener('DOMContentLoaded', async function() {
  try {
    const response = await fetch("http://localhost:2388")
  } catch (e) {
    const msg = document.querySelector("#message")
    msg.innerText = "daemon not found..."
  }
});
