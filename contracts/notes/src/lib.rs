#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data laporan
#[contracttype]
#[derive(Clone, Debug)]
pub struct Report {
    id: u64,
    title: String,
    description: String,
    status: String, // "pending", "proses", "selesai"
}

// ✅ FIX: max 9 karakter
const REPORT_DATA: Symbol = symbol_short!("REPORTS");

#[contract]
pub struct ComplaintContract;

#[contractimpl]
impl ComplaintContract {

    // Ambil semua laporan
    pub fn get_reports(env: Env) -> Vec<Report> {
        env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Membuat laporan baru
    pub fn create_report(env: Env, title: String, description: String) -> String {
        let mut reports: Vec<Report> = env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        let id = reports.len() as u64 + 1;

        let report = Report {
            id,
            title,
            description,
            status: String::from_str(&env, "pending"),
        };

        reports.push_back(report);
        env.storage().instance().set(&REPORT_DATA, &reports);

        String::from_str(&env, "Laporan berhasil dibuat")
    }

    // Update status laporan
    pub fn update_status(env: Env, id: u64, new_status: String) -> String {
        let mut reports: Vec<Report> = env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..reports.len() {
            let mut report = reports.get(i).unwrap();

            if report.id == id {
                report.status = new_status;
                reports.set(i, report);

                env.storage().instance().set(&REPORT_DATA, &reports);
                return String::from_str(&env, "Status berhasil diperbarui");
            }
        }

        String::from_str(&env, "Laporan tidak ditemukan")
    }

    // Hapus laporan
    pub fn delete_report(env: Env, id: u64) -> String {
        let mut reports: Vec<Report> = env.storage()
            .instance()
            .get(&REPORT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..reports.len() {
            if reports.get(i).unwrap().id == id {
                reports.remove(i);
                env.storage().instance().set(&REPORT_DATA, &reports);
                return String::from_str(&env, "Laporan berhasil dihapus");
            }
        }

        String::from_str(&env, "Laporan tidak ditemukan")
    }
}

mod test;