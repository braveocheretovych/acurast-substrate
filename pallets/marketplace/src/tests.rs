#![cfg(test)]

use frame_support::{
	assert_err, assert_ok,
	sp_runtime::{bounded_vec, Permill},
	traits::{Hooks, TypedGet},
};

use pallet_acurast::{
	utils::validate_and_extract_attestation, Attestation, JobModules, JobRegistrationFor,
	MultiOrigin, Schedule,
};
use reputation::{BetaReputation, ReputationEngine};

use crate::{
	mock::*, payments::JobBudget, stub::*, AdvertisementRestriction, Assignment,
	AssignmentStrategy, Config, Error, ExecutionMatch, ExecutionResult, ExecutionSpecifier,
	FeeManager, JobRequirements, JobStatus, Match, PlannedExecution, PlannedExecutions, PubKeys,
	RegistrationExtra, SLA,
};

/// Job is not assigned and gets deregistered successfully.
#[test]
fn test_valid_deregister() {
	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();

		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);
		assert_eq!(
			Some(ad.pricing.clone()),
			AcurastMarketplace::stored_advertisement_pricing(processor_account_id())
		);

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));
		assert_eq!(12_000_000, AcurastMarketplace::reserved(&job_id1));
		assert_eq!(
			Some(JobStatus::Open),
			AcurastMarketplace::stored_job_status(
				MultiOrigin::Acurast(alice_account_id()),
				initial_job_id + 1
			)
		);
		assert_eq!(
			Some(100_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(Acurast::deregister(
			RuntimeOrigin::signed(alice_account_id()).into(),
			job_id1.1,
		));

		assert_eq!(None, AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),);

		// the remaining budget got refunded
		assert_eq!(0, AcurastMarketplace::reserved(&job_id1));

		assert_eq!(
			events(),
			[
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone(),
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: alice_account_id(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationRemoved(
					job_id1.clone()
				)),
			]
		);
	});
}

#[test]
fn test_deregister_on_matched_job() {
	let now: u64 = 1_671_800_100_000; // 23.12.2022 12:55;

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(Some(bounded_vec![
					PlannedExecution { source: processor_account_id(), start_delay: 0 },
					PlannedExecution { source: processor_2_account_id(), start_delay: 0 }
				])),
				slots: 2,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();

		// pretend current time
		later(now);

		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_2_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);
		assert_eq!(
			Some(ad.pricing.clone()),
			AcurastMarketplace::stored_advertisement_pricing(processor_account_id())
		);

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));
		assert_eq!(Balances::free_balance(&alice_account_id()), 76_000_000);

		assert_eq!(24_000_000, AcurastMarketplace::reserved(&job_id1));
		assert_eq!(
			Some(JobStatus::Matched),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);
		assert_eq!(
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(Acurast::deregister(
			RuntimeOrigin::signed(alice_account_id()).into(),
			job_id1.1
		));
		// The amount should have been refunded
		assert_eq!(Balances::free_balance(&alice_account_id()), 100_000_000);

		// Job got removed after the deregister call
		assert_eq!(None, AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),);

		// the full budget got refunded
		assert_eq!(0, AcurastMarketplace::reserved(&job_id1));

		assert_eq!(
			events(),
			[
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_2_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationMatched(Match {
					job_id: job_id1.clone(),
					sources: bounded_vec![
						PlannedExecution { source: processor_account_id(), start_delay: 0 },
						PlannedExecution { source: processor_2_account_id(), start_delay: 0 }
					],
				})),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 24_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone(),
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: alice_account_id(),
					amount: 24_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationRemoved(
					job_id1.clone()
				)),
			]
		);
	});
}

#[test]
fn test_deregister_on_assigned_job() {
	let now: u64 = 1_671_800_100_000; // 23.12.2022 12:55;

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 0,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(Some(bounded_vec![
					PlannedExecution { source: processor_account_id(), start_delay: 0 },
					PlannedExecution { source: processor_2_account_id(), start_delay: 0 }
				])),
				slots: 2,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();

		// pretend current time
		later(now);

		let _ = Balances::force_set_balance(RuntimeOrigin::root(), alice_account_id(), 100_000_000);

		let consumer_initial_balance = 100_000_000u128;
		let processor_initial_balance = 10_000_000u128;
		let pallet_initial_balance = 10_000_000u128;

		assert_eq!(Balances::free_balance(&alice_account_id()), consumer_initial_balance);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&processor_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&pallet_acurast_acount()), pallet_initial_balance);
		assert_eq!(Balances::free_balance(&pallet_fees_account()), pallet_initial_balance);

		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_2_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);
		assert_eq!(
			Some(ad.pricing.clone()),
			AcurastMarketplace::stored_advertisement_pricing(processor_account_id())
		);

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));

		assert_eq!(24_000_000, AcurastMarketplace::reserved(&job_id1));
		assert_eq!(
			Some(JobStatus::Matched),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);
		assert_eq!(
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(AcurastMarketplace::acknowledge_match(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			PubKeys::default(),
		));
		let assignment =
			AcurastMarketplace::stored_matches(processor_account_id(), job_id1.clone()).unwrap();
		let total_reward = registration1.extra.requirements.reward *
			(registration1.extra.requirements.slots as u128) *
			(registration1.schedule.execution_count() as u128);
		assert_eq!(
			Balances::free_balance(&alice_account_id()),
			consumer_initial_balance - total_reward
		);
		// assert_eq!(Balances::free_balance(&alice_account_id()), 76_000_000);
		assert_eq!(Balances::free_balance(&processor_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);

		assert_ok!(Acurast::deregister(
			RuntimeOrigin::signed(alice_account_id()).into(),
			job_id1.1
		));
		assert_eq!(
			Balances::free_balance(&alice_account_id()),
			consumer_initial_balance - assignment.fee_per_execution
		);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);

		let fee_percentage = FeeManagerImpl::get_fee_percentage();
		let fee = fee_percentage.mul_floor(assignment.fee_per_execution);

		// Subtract the fee from the reward
		let reward_after_fee = assignment.fee_per_execution - fee;

		assert_eq!(
			Balances::free_balance(&processor_account_id()),
			processor_initial_balance + reward_after_fee
		);

		// Job got removed after the deregister call
		assert_eq!(None, AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),);

		// the full budget got refunded
		assert_eq!(0, AcurastMarketplace::reserved(&job_id1));

		assert_eq!(
			events(),
			[
				RuntimeEvent::Balances(pallet_balances::Event::BalanceSet {
					who: alice_account_id(),
					free: 100_000_000
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_2_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationMatched(Match {
					job_id: job_id1.clone(),
					sources: bounded_vec![
						PlannedExecution { source: processor_account_id(), start_delay: 0 },
						PlannedExecution { source: processor_2_account_id(), start_delay: 0 }
					],
				})),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: total_reward
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone(),
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id1.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: assignment.fee_per_execution,
						acknowledged: true,
						sla: SLA { total: 2, met: 0 },
						pub_keys: PubKeys::default()
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: fee
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: reward_after_fee
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: alice_account_id(),
					amount: total_reward - assignment.fee_per_execution
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationRemoved(
					job_id1.clone()
				)),
			]
		);
	});
}

#[test]
fn test_deregister_on_assigned_job_for_competing() {
	let now: u64 = 1_671_800_400_000 - <Test as Config>::MatchingCompetingDueDelta::get();

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 0,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Competing,
				slots: 2,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();

		// pretend current time
		later(now);

		let _ = Balances::force_set_balance(RuntimeOrigin::root(), alice_account_id(), 100_000_000);

		let consumer_initial_balance = 100_000_000u128;
		let processor_initial_balance = 10_000_000u128;
		let pallet_initial_balance = 10_000_000u128;

		assert_eq!(Balances::free_balance(&alice_account_id()), consumer_initial_balance);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&processor_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&pallet_acurast_acount()), pallet_initial_balance);
		assert_eq!(Balances::free_balance(&pallet_fees_account()), pallet_initial_balance);

		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_2_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);
		assert_eq!(
			Some(ad.pricing.clone()),
			AcurastMarketplace::stored_advertisement_pricing(processor_account_id())
		);

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));

		assert_eq!(24_000_000, AcurastMarketplace::reserved(&job_id1));
		assert_ok!(AcurastMarketplace::propose_execution_matching(
			RuntimeOrigin::signed(alice_account_id()).into(),
			vec![ExecutionMatch {
				job_id: job_id1.clone(),
				execution_index: 0,
				sources: vec![
					PlannedExecution { source: processor_account_id(), start_delay: 0 },
					PlannedExecution { source: processor_2_account_id(), start_delay: 0 }
				]
				.try_into()
				.unwrap(),
			}]
			.try_into()
			.unwrap(),
		));
		assert_eq!(
			Some(JobStatus::Matched),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);
		assert_eq!(
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(AcurastMarketplace::acknowledge_execution_match(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			0,
			PubKeys::default(),
		));
		assert_ok!(AcurastMarketplace::acknowledge_execution_match(
			RuntimeOrigin::signed(processor_2_account_id()).into(),
			job_id1.clone(),
			0,
			PubKeys::default(),
		));
		let assignment1 =
			AcurastMarketplace::stored_matches(processor_account_id(), job_id1.clone()).unwrap();
		let assignment2 =
			AcurastMarketplace::stored_matches(processor_2_account_id(), job_id1.clone()).unwrap();
		let total_reward = registration1.extra.requirements.reward *
			(registration1.extra.requirements.slots as u128) *
			(registration1.schedule.execution_count() as u128);
		assert_eq!(
			Balances::free_balance(&alice_account_id()),
			consumer_initial_balance - total_reward
		);
		assert_eq!(Balances::free_balance(&processor_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);

		assert_ok!(Acurast::deregister(
			RuntimeOrigin::signed(alice_account_id()).into(),
			job_id1.1
		));

		let fee_percentage = FeeManagerImpl::get_fee_percentage();
		let fee1 = fee_percentage.mul_floor(assignment1.fee_per_execution);
		let fee2 = fee_percentage.mul_floor(assignment2.fee_per_execution);

		// Subtract the fee from the reward
		let reward1_after_fee = assignment1.fee_per_execution - fee1;
		let reward2_after_fee = assignment1.fee_per_execution - fee2;

		assert_eq!(
			Balances::free_balance(&alice_account_id()),
			consumer_initial_balance -
				(assignment1.fee_per_execution + assignment2.fee_per_execution)
		);
		assert_eq!(
			Balances::free_balance(&processor_2_account_id()),
			processor_initial_balance + reward2_after_fee
		);
		assert_eq!(
			Balances::free_balance(&processor_account_id()),
			processor_initial_balance + reward1_after_fee
		);

		// Job got removed after the deregister call
		assert_eq!(None, AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),);

		// the full budget got refunded
		assert_eq!(0, AcurastMarketplace::reserved(&job_id1));

		assert_eq!(
			events(),
			[
				RuntimeEvent::Balances(pallet_balances::Event::BalanceSet {
					who: alice_account_id(),
					free: consumer_initial_balance
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_2_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: total_reward
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone(),
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobExecutionMatched(
					ExecutionMatch {
						job_id: job_id1.clone(),
						execution_index: 0,
						sources: bounded_vec![
							PlannedExecution { source: processor_account_id(), start_delay: 0 },
							PlannedExecution { source: processor_2_account_id(), start_delay: 0 }
						],
					}
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id1.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::Index(0),
						start_delay: 0,
						fee_per_execution: assignment1.fee_per_execution,
						acknowledged: true,
						sla: SLA { total: 1, met: 0 },
						pub_keys: PubKeys::default()
					}
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id1.clone(),
					processor_2_account_id(),
					Assignment {
						slot: 1,
						execution: ExecutionSpecifier::Index(0),
						start_delay: 0,
						fee_per_execution: assignment2.fee_per_execution,
						acknowledged: true,
						sla: SLA { total: 1, met: 0 },
						pub_keys: PubKeys::default()
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: fee1
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: reward1_after_fee
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: fee2
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_2_account_id(),
					amount: reward2_after_fee
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: alice_account_id(),
					amount: total_reward -
						(assignment1.fee_per_execution + assignment2.fee_per_execution)
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationRemoved(
					job_id1.clone()
				)),
			]
		);
	});
}

#[test]
fn test_deregister_on_assigned_job_for_competing_2() {
	let now: u64 = 1_671_800_400_000 - <Test as Config>::MatchingCompetingDueDelta::get();

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 0,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Competing,
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();

		// pretend current time
		later(now);

		let _ = Balances::force_set_balance(RuntimeOrigin::root(), alice_account_id(), 100_000_000);

		let consumer_initial_balance = 100_000_000u128;
		let processor_initial_balance = 10_000_000u128;
		let pallet_initial_balance = 10_000_000u128;

		assert_eq!(Balances::free_balance(&alice_account_id()), consumer_initial_balance);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&processor_account_id()), processor_initial_balance);
		assert_eq!(Balances::free_balance(&pallet_acurast_acount()), pallet_initial_balance);
		assert_eq!(Balances::free_balance(&pallet_fees_account()), pallet_initial_balance);

		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_2_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);
		assert_eq!(
			Some(ad.pricing.clone()),
			AcurastMarketplace::stored_advertisement_pricing(processor_account_id())
		);

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));

		assert_eq!(12_000_000, AcurastMarketplace::reserved(&job_id1));
		assert_ok!(AcurastMarketplace::propose_execution_matching(
			RuntimeOrigin::signed(alice_account_id()).into(),
			vec![ExecutionMatch {
				job_id: job_id1.clone(),
				execution_index: 0,
				sources: vec![PlannedExecution { source: processor_account_id(), start_delay: 0 },]
					.try_into()
					.unwrap(),
			}]
			.try_into()
			.unwrap(),
		));
		assert_eq!(
			Some(JobStatus::Matched),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);
		assert_eq!(
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(AcurastMarketplace::acknowledge_execution_match(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			0,
			PubKeys::default(),
		));

		later(registration1.schedule.start_time);

		assert_ok!(AcurastMarketplace::report(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			ExecutionResult::Success(b"JOB_EXECUTED".to_vec().try_into().unwrap()),
		));

		later(now + registration1.schedule.interval);

		assert_ok!(AcurastMarketplace::propose_execution_matching(
			RuntimeOrigin::signed(alice_account_id()).into(),
			vec![ExecutionMatch {
				job_id: job_id1.clone(),
				execution_index: 1,
				sources: vec![PlannedExecution {
					source: processor_2_account_id(),
					start_delay: 0,
				},]
				.try_into()
				.unwrap(),
			}]
			.try_into()
			.unwrap(),
		));

		assert_ok!(AcurastMarketplace::acknowledge_execution_match(
			RuntimeOrigin::signed(processor_2_account_id()).into(),
			job_id1.clone(),
			1,
			PubKeys::default(),
		));

		let assignment1 =
			AcurastMarketplace::stored_matches(processor_account_id(), job_id1.clone()).unwrap();
		let assignment2 =
			AcurastMarketplace::stored_matches(processor_2_account_id(), job_id1.clone()).unwrap();
		let total_reward = registration1.extra.requirements.reward *
			(registration1.extra.requirements.slots as u128) *
			(registration1.schedule.execution_count() as u128);
		let fee_percentage = FeeManagerImpl::get_fee_percentage();
		let fee1 = fee_percentage.mul_floor(assignment1.fee_per_execution);
		let reward1_after_fee = assignment1.fee_per_execution - fee1;

		assert_eq!(
			Balances::free_balance(&alice_account_id()),
			consumer_initial_balance - total_reward
		);
		assert_eq!(
			Balances::free_balance(&processor_account_id()),
			processor_initial_balance + reward1_after_fee
		);
		assert_eq!(Balances::free_balance(&processor_2_account_id()), processor_initial_balance);

		assert_ok!(Acurast::deregister(
			RuntimeOrigin::signed(alice_account_id()).into(),
			job_id1.1
		));

		let fee2 = fee_percentage.mul_floor(assignment2.fee_per_execution);
		let reward2_after_fee = assignment2.fee_per_execution - fee2;

		assert_eq!(
			Balances::free_balance(&alice_account_id()),
			consumer_initial_balance -
				(assignment1.fee_per_execution + assignment2.fee_per_execution)
		);
		assert_eq!(
			Balances::free_balance(&processor_2_account_id()),
			processor_initial_balance + reward2_after_fee
		);
		assert_eq!(
			Balances::free_balance(&processor_account_id()),
			processor_initial_balance + reward1_after_fee
		);

		// Job got removed after the deregister call
		assert_eq!(None, AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),);

		// the full budget got refunded
		assert_eq!(0, AcurastMarketplace::reserved(&job_id1));

		assert_eq!(
			events(),
			[
				RuntimeEvent::Balances(pallet_balances::Event::BalanceSet {
					who: alice_account_id(),
					free: consumer_initial_balance
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_2_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: total_reward
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone(),
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobExecutionMatched(
					ExecutionMatch {
						job_id: job_id1.clone(),
						execution_index: 0,
						sources: bounded_vec![PlannedExecution {
							source: processor_account_id(),
							start_delay: 0
						},],
					}
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id1.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::Index(0),
						start_delay: 0,
						fee_per_execution: assignment1.fee_per_execution,
						acknowledged: true,
						sla: SLA { total: 1, met: 0 },
						pub_keys: PubKeys::default()
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: fee1
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: reward1_after_fee
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::ExecutionSuccess(
					job_id1.clone(),
					b"JOB_EXECUTED".to_vec().try_into().unwrap()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::Reported(
					job_id1.clone(),
					processor_account_id(),
					assignment1.clone()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobExecutionMatched(
					ExecutionMatch {
						job_id: job_id1.clone(),
						execution_index: 1,
						sources: bounded_vec![PlannedExecution {
							source: processor_2_account_id(),
							start_delay: 0
						},],
					}
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id1.clone(),
					processor_2_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::Index(1),
						start_delay: 0,
						fee_per_execution: assignment2.fee_per_execution,
						acknowledged: true,
						sla: SLA { total: 1, met: 0 },
						pub_keys: PubKeys::default()
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: fee2
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_2_account_id(),
					amount: reward2_after_fee
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: alice_account_id(),
					amount: total_reward -
						(assignment1.fee_per_execution + assignment2.fee_per_execution)
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationRemoved(
					job_id1.clone()
				)),
			]
		);
	});
}

#[test]
fn test_match() {
	let now: u64 = 1_671_800_100_000; // 23.12.2022 12:55;

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};
	let registration2 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 10_000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();

		// pretend current time
		later(now);

		let chain = attestation_chain();
		assert_ok!(Acurast::submit_attestation(
			RuntimeOrigin::signed(processor_account_id()).into(),
			chain.clone()
		));
		let attestation =
			validate_and_extract_attestation::<Test>(&processor_account_id(), &chain).unwrap();

		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);
		assert_eq!(
			Some(ad.pricing.clone()),
			AcurastMarketplace::stored_advertisement_pricing(processor_account_id())
		);

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);
		let job_id2 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 2);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));
		assert_eq!(12_000_000, AcurastMarketplace::reserved(&job_id1));
		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration2.clone(),
		));
		assert_eq!(12_000_000, AcurastMarketplace::reserved(&job_id2));
		assert_eq!(
			Some(JobStatus::Open),
			AcurastMarketplace::stored_job_status(
				MultiOrigin::Acurast(alice_account_id()),
				initial_job_id + 1
			)
		);
		assert_eq!(
			Some(100_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		let job_match1 = Match {
			job_id: job_id1.clone(),
			sources: bounded_vec![PlannedExecution {
				source: processor_account_id(),
				start_delay: 0,
			}],
		};
		let job_match2 = Match {
			job_id: job_id2.clone(),
			sources: bounded_vec![PlannedExecution {
				source: processor_account_id(),
				start_delay: 5_000,
			}],
		};

		assert_ok!(AcurastMarketplace::propose_matching(
			RuntimeOrigin::signed(charlie_account_id()).into(),
			vec![job_match1.clone(), job_match2.clone()].try_into().unwrap(),
		));
		assert_eq!(
			Some(JobStatus::Matched),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);
		assert_eq!(
			Some(60_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);
		// matcher got paid out already so job budget decreased
		assert_eq!(11804000, AcurastMarketplace::reserved(&job_id1));
		assert_eq!(11804000, AcurastMarketplace::reserved(&job_id2));

		assert_ok!(AcurastMarketplace::acknowledge_match(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			PubKeys::default(),
		));
		assert_eq!(
			Some(JobStatus::Assigned(1)),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);

		// pretend time moved on
		assert_eq!(1, System::block_number());
		later(registration1.schedule.start_time + 3000); // pretend actual execution until report call took 3 seconds
		assert_eq!(2, System::block_number());

		assert_ok!(AcurastMarketplace::report(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			ExecutionResult::Success(operation_hash())
		));
		// job budget decreased by reward worth one execution
		assert_eq!(6784000, AcurastMarketplace::reserved(&job_id1));
		// average reward only updated at end of job
		assert_eq!(None, AcurastMarketplace::average_reward());
		// reputation still ~50%
		assert_eq!(
			Permill::from_parts(509_803),
			BetaReputation::<u128>::normalize(
				AcurastMarketplace::stored_reputation(processor_account_id()).unwrap()
			)
			.unwrap()
		);
		assert_eq!(
			Some(Assignment {
				execution: ExecutionSpecifier::All,
				slot: 0,
				start_delay: 0,
				fee_per_execution: 5_020_000,
				acknowledged: true,
				sla: SLA { total: 2, met: 1 },
				pub_keys: PubKeys::default(),
			}),
			AcurastMarketplace::stored_matches(processor_account_id(), job_id1.clone()),
		);
		// Job still assigned after one execution
		assert_eq!(
			Some(JobStatus::Assigned(1)),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),
		);
		assert_eq!(
			Some(60000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		// pretend time moved on
		later(registration1.schedule.range(0).unwrap().1 - 2000);
		assert_eq!(3, System::block_number());

		assert_ok!(AcurastMarketplace::report(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			ExecutionResult::Success(operation_hash())
		));
		// job budget decreased by reward worth one execution
		assert_eq!(1764000, AcurastMarketplace::reserved(&job_id1));

		// pretend time moved on
		later(registration1.schedule.end_time + 1);
		assert_eq!(4, System::block_number());

		assert_eq!(1764000, AcurastMarketplace::reserved(&job_id1));

		assert_ok!(AcurastMarketplace::finalize_job(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone()
		));

		assert_eq!(
			None,
			AcurastMarketplace::stored_matches(processor_account_id(), job_id1.clone()),
		);
		assert_eq!(Some(2), AcurastMarketplace::total_assigned());
		// average reward only updated at end of job
		assert_eq!(Some(2510000), AcurastMarketplace::average_reward());
		// reputation increased
		assert_eq!(
			Permill::from_parts(763_424),
			BetaReputation::<u128>::normalize(
				AcurastMarketplace::stored_reputation(processor_account_id()).unwrap()
			)
			.unwrap()
		);
		// Job still assigned after last execution
		assert_eq!(
			Some(JobStatus::Assigned(1)),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),
		);
		assert_eq!(
			// only job2 is still blocking memory
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(AcurastMarketplace::finalize_jobs(
			RuntimeOrigin::signed(alice_account_id()).into(),
			vec![job_id1.1].try_into().unwrap(),
		));

		// Job no longer assigned after finalization
		assert_eq!(None, AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1),);
		// the remaining budget got refunded
		assert_eq!(0, AcurastMarketplace::reserved(&job_id1));
		// but job2 still have full budget
		assert_eq!(11804000, AcurastMarketplace::reserved(&job_id2));

		assert_eq!(
			events(),
			[
				RuntimeEvent::Acurast(pallet_acurast::Event::AttestationStored(
					attestation,
					processor_account_id()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone(),
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration2.clone(),
					job_id2.clone(),
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationMatched(job_match1)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationMatched(job_match2)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 117_600
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: charlie_account_id(),
					amount: 274_400
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id1.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: 5_020_000,
						acknowledged: true,
						sla: SLA { total: 2, met: 0 },
						pub_keys: PubKeys::default(),
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 1_506_000
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: 3_514_000
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::ExecutionSuccess(
					job_id1.clone(),
					operation_hash()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::Reported(
					job_id1.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: 5_020_000,
						acknowledged: true,
						sla: SLA { total: 2, met: 1 },
						pub_keys: PubKeys::default(),
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 1_506_000
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: 3_514_000
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::ExecutionSuccess(
					job_id1.clone(),
					operation_hash()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::Reported(
					job_id1.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: 5_020_000,
						acknowledged: true,
						sla: SLA { total: 2, met: 2 },
						pub_keys: PubKeys::default(),
					}
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobFinalized(job_id1.clone())),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: alice_account_id(),
					amount: 1_764_000
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobFinalized(job_id1.clone(),)),
			]
		);
	});
}

#[test]
fn test_multi_assignments() {
	let now = 1_694_795_700_000; // 15.09.2023 17:35

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 1000,
			start_time: 1_694_796_000_000, // 15.09.2023 17:40
			end_time: 1_694_796_120_000,   // 15.09.2023 17:42 (2 minutes later)
			interval: 10000,               // 10 seconds
			max_start_delay: 0,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 4,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let _ =
			Balances::force_set_balance(RuntimeOrigin::root(), alice_account_id(), 1_000_000_000);

		let initial_job_id = Acurast::job_id_sequence();

		// pretend current time
		later(now);

		let processors = vec![
			(processor_account_id(), attestation_chain()),
			(processor_2_account_id(), attestation_chain_processor_2()),
			(processor_3_account_id(), attestation_chain_processor_3()),
			(processor_4_account_id(), attestation_chain_processor_4()),
		];

		let _attestations: Vec<Attestation> = processors
			.iter()
			.map(|(processor, attestation_chain)| {
				assert_ok!(Acurast::submit_attestation(
					RuntimeOrigin::signed(processor.clone()).into(),
					attestation_chain.clone()
				));
				let attestation =
					validate_and_extract_attestation::<Test>(processor, &attestation_chain)
						.unwrap();

				assert_ok!(AcurastMarketplace::advertise(
					RuntimeOrigin::signed(processor.clone()).into(),
					ad.clone(),
				));
				assert_eq!(
					Some(AdvertisementRestriction {
						max_memory: 50_000,
						network_request_quota: 8,
						storage_capacity: 100_000,
						allowed_consumers: ad.allowed_consumers.clone(),
						available_modules: JobModules::default(),
						cpu_score: 1
					}),
					AcurastMarketplace::stored_advertisement(processor)
				);
				assert_eq!(
					Some(ad.pricing.clone()),
					AcurastMarketplace::stored_advertisement_pricing(processor)
				);

				return attestation
			})
			.collect();

		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration.clone(),
		));
		assert_eq!(
			Some(JobStatus::Open),
			AcurastMarketplace::stored_job_status(
				MultiOrigin::Acurast(alice_account_id()),
				initial_job_id + 1
			)
		);

		let job_sources: PlannedExecutions<AccountId, <Test as pallet_acurast::Config>::MaxSlots> =
			processors
				.iter()
				.map(|(processor, _)| PlannedExecution {
					source: processor.clone(),
					start_delay: 0,
				})
				.collect::<Vec<PlannedExecution<AccountId>>>()
				.try_into()
				.unwrap();

		let job_match = Match { job_id: job_id1.clone(), sources: job_sources };

		assert_ok!(AcurastMarketplace::propose_matching(
			RuntimeOrigin::signed(charlie_account_id()).into(),
			vec![job_match.clone()].try_into().unwrap(),
		));

		assert_eq!(
			Some(JobStatus::Matched),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);
		assert_eq!(
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);
		// matcher got rewarded already so job budget decreased
		assert_eq!(264096000, AcurastMarketplace::reserved(&job_id1));

		// pretend current time
		let mut start_time = registration.schedule.start_time;
		processors.iter().for_each(|(processor, _)| {
			start_time += 6000;
			later(start_time);
			assert_ok!(AcurastMarketplace::acknowledge_match(
				RuntimeOrigin::signed(processor.clone()).into(),
				job_id1.clone(),
				PubKeys::default(),
			));
		});

		assert_eq!(
			Some(JobStatus::Assigned(processors.len() as u8)),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);

		// job budget decreased by reward worth one execution
		assert_eq!(264096000, AcurastMarketplace::reserved(&job_id1));
		// average reward only updated at end of job
		assert_eq!(None, AcurastMarketplace::average_reward());
		// reputation still ~50%
		assert_eq!(
			Permill::from_parts(509_803),
			BetaReputation::<u128>::normalize(
				AcurastMarketplace::stored_reputation(processor_account_id()).unwrap()
			)
			.unwrap()
		);
		processors.iter().zip(0..processors.len()).for_each(|((processor, _), slot)| {
			assert_eq!(
				Some(Assignment {
					execution: ExecutionSpecifier::All,
					slot: slot as u8,
					start_delay: 0,
					fee_per_execution: 1_020_000,
					acknowledged: true,
					sla: SLA { total: 12, met: 0 },
					pub_keys: PubKeys::default(),
				}),
				AcurastMarketplace::stored_matches(processor, job_id1.clone()),
			);
			assert_ok!(AcurastMarketplace::report(
				RuntimeOrigin::signed(processor.clone()).into(),
				job_id1.clone(),
				ExecutionResult::Success(operation_hash())
			));
			assert_eq!(
				Some(Assignment {
					execution: ExecutionSpecifier::All,
					slot: slot as u8,
					start_delay: 0,
					fee_per_execution: 1_020_000,
					acknowledged: true,
					sla: SLA { total: 12, met: 1 },
					pub_keys: PubKeys::default(),
				}),
				AcurastMarketplace::stored_matches(processor, job_id1.clone()),
			);
		});

		// Processors are still assigned after one execution
		assert_eq!(
			Some(JobStatus::Assigned(processors.len() as u8)),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);

		assert_eq!(
			Some(80_000),
			AcurastMarketplace::stored_storage_capacity(processor_account_id())
		);

		assert_ok!(AcurastMarketplace::report(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id1.clone(),
			ExecutionResult::Success(operation_hash())
		));
		// job budget decreased by reward worth one execution
		assert_eq!(258996000, AcurastMarketplace::reserved(&job_id1));

		// pretend time moved on
		later(registration.schedule.end_time + 1);

		assert_eq!(258996000, AcurastMarketplace::reserved(&job_id1));

		processors.iter().for_each(|(processor, _)| {
			assert_ok!(AcurastMarketplace::finalize_job(
				RuntimeOrigin::signed(processor.clone()).into(),
				job_id1.clone()
			));
			assert_eq!(None, AcurastMarketplace::stored_matches(processor, job_id1.clone()),)
		});

		assert_eq!(Some(1), AcurastMarketplace::total_assigned());
	});
}

#[test]
fn test_no_match_schedule_overlap() {
	let now: u64 = 1_671_800_100_000; // 23.12.2022 12:55;

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min -> 2 executions fit
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	let registration2 = JobRegistrationFor::<Test> {
		script: script_random_value(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_802_200_000, // 23.12.2022 13:30
			end_time: 1_671_805_800_000,   // 23.12.2022 14:30 (one hour later)
			interval: 1_200_000,           // 20min -> 3 executions fit
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();
		let job_id1 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);
		let job_id2 = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 2);

		// pretend current time
		assert_ok!(Timestamp::set(RuntimeOrigin::none(), now));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));

		// register first job
		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));
		assert_eq!(
			Some(JobStatus::Open),
			AcurastMarketplace::stored_job_status(&job_id1.0, &job_id1.1)
		);

		// register second job
		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration2.clone(),
		));
		assert_eq!(
			Some(JobStatus::Open),
			AcurastMarketplace::stored_job_status(&job_id1.0, job_id1.1 + 1)
		);

		// the first job matches because capacity left
		let m = Match {
			job_id: job_id1.clone(),
			sources: bounded_vec![PlannedExecution {
				source: processor_account_id(),
				start_delay: 0,
			}],
		};
		assert_ok!(AcurastMarketplace::propose_matching(
			RuntimeOrigin::signed(charlie_account_id()).into(),
			vec![m.clone()].try_into().unwrap(),
		));

		// this one does not match anymore
		let m2 = Match {
			job_id: job_id2.clone(),
			sources: bounded_vec![PlannedExecution {
				source: processor_account_id(),
				start_delay: 0,
			}],
		};
		assert_err!(
			AcurastMarketplace::propose_matching(
				RuntimeOrigin::signed(charlie_account_id()).into(),
				vec![m2.clone()].try_into().unwrap(),
			),
			Error::<Test>::ScheduleOverlapInMatch
		);

		assert_eq!(
			events(),
			[
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id1.clone()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 18_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration2.clone(),
					(job_id2.0.clone(), job_id2.1.clone())
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationMatched(m)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 58800
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: charlie_account_id(),
					amount: 137200
				}),
				// no match event for second
			]
		);
	});
}

#[test]
fn test_no_match_insufficient_reputation() {
	let now: u64 = 1_671_800_100_000; // 23.12.2022 12:55;

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration1 = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min -> 2 executions fit
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: Some(1_000_000),
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();
		let job_id = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		// pretend current time
		assert_ok!(Timestamp::set(RuntimeOrigin::none(), now));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));

		// register job
		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration1.clone(),
		));
		assert_eq!(
			Some(JobStatus::Open),
			AcurastMarketplace::stored_job_status(&job_id.0, job_id.1)
		);

		// the job matches except insufficient reputation
		let m = Match {
			job_id: job_id.clone(),
			sources: bounded_vec![PlannedExecution {
				source: processor_account_id(),
				start_delay: 0,
			}],
		};
		assert_err!(
			AcurastMarketplace::propose_matching(
				RuntimeOrigin::signed(charlie_account_id()).into(),
				vec![m.clone()].try_into().unwrap(),
			),
			Error::<Test>::InsufficientReputationInMatch
		);

		assert_eq!(
			events(),
			[
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration1.clone(),
					job_id.clone()
				)),
				// no match event for job
			]
		);
	});
}

#[test]
fn test_more_reports_than_expected() {
	let now: u64 = 1_671_800_100_000; // 23.12.2022 12:55;

	// 1000 is the smallest amount accepted by T::AssetTransactor::lock_asset for the asset used
	let ad = advertisement(1000, 1, 100_000, 50_000, 8);
	let registration = JobRegistrationFor::<Test> {
		script: script(),
		allowed_sources: None,
		allow_only_verified_sources: false,
		schedule: Schedule {
			duration: 5000,
			start_time: 1_671_800_400_000, // 23.12.2022 13:00
			end_time: 1_671_804_000_000,   // 23.12.2022 14:00 (one hour later)
			interval: 1_800_000,           // 30min
			max_start_delay: 5000,
		},
		memory: 5_000u32,
		network_requests: 5,
		storage: 20_000u32,
		required_modules: JobModules::default(),
		extra: RegistrationExtra {
			requirements: JobRequirements {
				assignment_strategy: AssignmentStrategy::Single(None),
				slots: 1,
				reward: 3_000_000 * 2,
				min_reputation: None,
				processor_version: None,
				min_cpu_score: None,
			},
		},
	};

	ExtBuilder::default().build().execute_with(|| {
		let initial_job_id = Acurast::job_id_sequence();
		let job_id = (MultiOrigin::Acurast(alice_account_id()), initial_job_id + 1);

		// pretend current time
		assert_ok!(Timestamp::set(RuntimeOrigin::none(), now));
		assert_ok!(AcurastMarketplace::advertise(
			RuntimeOrigin::signed(processor_account_id()).into(),
			ad.clone(),
		));
		assert_eq!(
			Some(AdvertisementRestriction {
				max_memory: 50_000,
				network_request_quota: 8,
				storage_capacity: 100_000,
				allowed_consumers: ad.allowed_consumers.clone(),
				available_modules: JobModules::default(),
				cpu_score: 1
			}),
			AcurastMarketplace::stored_advertisement(processor_account_id())
		);

		assert_ok!(Acurast::register(
			RuntimeOrigin::signed(alice_account_id()).into(),
			registration.clone(),
		));

		let m = Match {
			job_id: job_id.clone(),
			sources: bounded_vec![PlannedExecution {
				source: processor_account_id(),
				start_delay: 0,
			}],
		};
		assert_ok!(AcurastMarketplace::propose_matching(
			RuntimeOrigin::signed(charlie_account_id()).into(),
			vec![m.clone()].try_into().unwrap(),
		));

		assert_ok!(AcurastMarketplace::acknowledge_match(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id.clone(),
			PubKeys::default(),
		));

		// report twice with success
		// -------------------------

		// pretend time moved on
		let mut iter = registration.schedule.iter(0).unwrap();
		later(iter.next().unwrap() + 1000);
		assert_ok!(AcurastMarketplace::report(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id.clone(),
			ExecutionResult::Success(operation_hash())
		));

		// pretend time moved on
		later(iter.next().unwrap() + 1000);
		assert_ok!(AcurastMarketplace::report(
			RuntimeOrigin::signed(processor_account_id()).into(),
			job_id.clone(),
			ExecutionResult::Success(operation_hash())
		));

		// third report is illegal!
		later(registration.schedule.range(0).unwrap().1 + 1000);
		assert_err!(
			AcurastMarketplace::report(
				RuntimeOrigin::signed(processor_account_id()).into(),
				job_id.clone(),
				ExecutionResult::Success(operation_hash())
			),
			Error::<Test>::MoreReportsThanExpected
		);

		assert_eq!(
			events(),
			[
				RuntimeEvent::AcurastMarketplace(crate::Event::AdvertisementStored(
					ad.clone(),
					processor_account_id()
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: alice_account_id(),
					to: pallet_acurast_acount(),
					amount: 12_000_000
				}),
				RuntimeEvent::Acurast(pallet_acurast::Event::JobRegistrationStored(
					registration.clone(),
					job_id.clone()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationMatched(m)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 58_800
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: charlie_account_id(),
					amount: 137_200
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::JobRegistrationAssigned(
					job_id.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: 5_020_000,
						acknowledged: true,
						sla: SLA { total: 2, met: 0 },
						pub_keys: PubKeys::default(),
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 1_506_000
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: 3_514_000
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::ExecutionSuccess(
					job_id.clone(),
					operation_hash()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::Reported(
					job_id.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: 5_020_000,
						acknowledged: true,
						sla: SLA { total: 2, met: 1 },
						pub_keys: PubKeys::default(),
					}
				)),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: pallet_fees_account(),
					amount: 1_506_000
				}),
				RuntimeEvent::Balances(pallet_balances::Event::Transfer {
					from: pallet_acurast_acount(),
					to: processor_account_id(),
					amount: 3_514_000
				}),
				RuntimeEvent::AcurastMarketplace(crate::Event::ExecutionSuccess(
					job_id.clone(),
					operation_hash()
				)),
				RuntimeEvent::AcurastMarketplace(crate::Event::Reported(
					job_id.clone(),
					processor_account_id(),
					Assignment {
						slot: 0,
						execution: ExecutionSpecifier::All,
						start_delay: 0,
						fee_per_execution: 5_020_000,
						acknowledged: true,
						sla: SLA { total: 2, met: 2 },
						pub_keys: PubKeys::default(),
					}
				)),
			]
		);
	});
}

fn next_block() {
	if System::block_number() >= 1 {
		// pallet_acurast_marketplace::on_finalize(System::block_number());
		Timestamp::on_finalize(System::block_number());
	}
	System::set_block_number(System::block_number() + 1);
	Timestamp::on_initialize(System::block_number());
}

/// A helper function to move time on in tests. It ensures `Timestamp::set` is only called once per block by advancing the block otherwise.
fn later(now: u64) {
	// If this is not the very first timestamp ever set, we always advance the block before setting new time
	// this is because setting it twice in a block is not legal
	if Timestamp::get() > 0 {
		// pretend block was finalized
		let b = System::block_number();
		next_block(); // we cannot set time twice in same block
		assert_eq!(b + 1, System::block_number());
	}
	// pretend time moved on
	assert_ok!(Timestamp::set(RuntimeOrigin::none(), now));
}
