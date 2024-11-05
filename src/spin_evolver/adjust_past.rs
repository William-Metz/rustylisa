
// Project: Gravitational Wave Visualizer
// Author: Will Metz(Pomona College)
// Created: 2024
// File Path: src/spin_evolver/adjust_past.rs
// ------------------------------------------------------------
// This project is open for educational and personal use only.
// No warranty is provided, and this is not an official product.
// ------------------------------------------------------------
use crate::spin_evolver::spin_evolver::SpinEvolverClass;
impl SpinEvolverClass {
    pub fn adjust_the_past(&mut self) {
        self.chi1x_p = 0.5 * (self.chi1x_n + self.chi1x_p);
        self.chi1y_p = 0.5 * (self.chi1y_n + self.chi1y_p);
        self.chi1z_p = 0.5 * (self.chi1z_n + self.chi1z_p);

        self.ell_x_p = 0.5 * (self.ell_x_n + self.ell_x_p);
        self.ell_y_p = 0.5 * (self.ell_y_n + self.ell_y_p);
        self.ell_z_p = 0.5 * (self.ell_z_n + self.ell_z_p);

        self.psi_pr_p = 0.5 * (self.psi_pr_n + self.psi_pr_p);
    }
}
