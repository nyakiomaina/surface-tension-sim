import { Simulation } from './pkg/surface_tension_sim.js';

function run() {
  const sim = new Simulation(100, 800.0, 600.0);

  const canvas = document.getElementById('simulationCanvas');
  const ctx = canvas.getContext('2d');

  function drawParticles(particles) {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    particles.forEach(p => {
      const speed = Math.sqrt(p.vx * p.vx + p.vy * p.vy);
      const hue = Math.min(speed * 10, 360);
      ctx.fillStyle = `hsl(${hue}, 100%, 50%)`;

      const radius = Math.min(speed * 2, 10);
      ctx.beginPath();
      ctx.arc(p.x, p.y, radius, 0, 2 * Math.PI);
      ctx.fill();
    });
  }

  function animate() {
    sim.step();
    const particles = sim.get_particles();
    drawParticles(particles);
    requestAnimationFrame(animate);
  }

  animate();
}

run();
