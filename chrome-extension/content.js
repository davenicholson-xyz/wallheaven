let DAEMON = false;

function throttle(fn, delay) {
  let lastCall = 0;

  return function(...args) {
    const now = new Date().getTime();

    if (now - lastCall >= delay) {
      lastCall = now;
      fn(...args);
    }
  };
}

async function getWallpaper(id) {
  try {
    let response = await fetch(`http://localhost:2388/wp/${id}`);
    if (!response.ok) {
      let data = await response.json();
      alert(data.error);
    }
  } catch (e) {
    console.error("a boo boo has happened")
  }
}

async function checkDaemon() {
  try {
    const response = await fetch("http://localhost:2388")
    if (response.ok) {
      return true
    }
  } catch (e) {
    return false
  }
}

function addThumbButtons() {
  let thumbs = document.querySelectorAll("figure.thumb");
  thumbs.forEach(thumb => {

    if (!thumb.querySelector(".thumb-btn-wallheaven")) {
      let link = thumb.getElementsByTagName("a")[0].getAttribute("href");
      let wp_id = link.split("/").pop();

      const button = document.createElement('a');
      button.classList.add("thumb-btn");
      button.classList.add("thumb-btn-wallheaven");
      button.classList.add("jsAnchor");
      button.classList.add("overlay-anchor");

      const i = document.createElement("i");
      i.classList.add("fas");
      i.classList.add("fa-paper-plane");
      button.appendChild(i);

      thumb.appendChild(button);

      button.addEventListener('click', async (e) => {
        e.stopPropagation();
        getWallpaper(wp_id);
      })
    }

  })
}

function addWallpaperButton() {

  let fav_button = document.querySelector("#fav-button");
  if (fav_button) {

    let link = document.getElementById("wallpaper-short-url-copy").value;
    let wp_id = link.split("/").pop();

    const i = document.createElement("i");
    i.classList.add("fas");
    i.classList.add("fa-paper-plane");

    const span = document.createElement("span");
    span.innerText = "Set as wallpaper";

    const button = document.createElement('a');
    button.classList.add("button");
    button.classList.add("add-button");
    button.classList.add("btn-set-wallpaper");
    button.appendChild(i);
    button.appendChild(span);

    fav_button.after(button);

    button.addEventListener('click', async (e) => {
      e.stopPropagation();
      getWallpaper(wp_id);
    })
  }

}

let throt_addThumbButtons = throttle(addThumbButtons, 500);

window.onload = async () => {
  DAEMON = await checkDaemon()
  if (DAEMON) {
    addThumbButtons();
    addWallpaperButton();
  } else {
    console.info("Wallheaven daemon not running...")
  }
}

window.onscroll = () => {
  if (DAEMON) {
    throt_addThumbButtons();
  }
}
